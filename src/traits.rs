use std::{
    convert::TryInto,
    iter::{Filter, Map},
};

use bevy::utils::hashbrown::{
    hash_map::{Drain, Values, ValuesMut},
    HashMap,
};

pub trait Reposit<T: ManageTarget> {
    fn manager(&self) -> &HashMap<String, T>;
    fn manager_mut(&mut self) -> &mut HashMap<String, T>;
}

pub trait ManageTarget {
    fn new<S: Into<String>, P: Into<String>>(label: S, path: P) -> Self;
    fn label(&self) -> &str;
}

pub trait Manage {
    // -------
    #[inline]
    fn iter<T: ManageTarget>(&self) -> Values<String, T>
    where
        Self: Reposit<T>,
    {
        self.manager().values()
    }
    // -------
    // -------
    #[inline]
    fn iter_mut<T: ManageTarget>(&mut self) -> ValuesMut<String, T>
    where
        Self: Reposit<T>,
    {
        self.manager_mut().values_mut()
    }
    // -------
    #[inline]
    fn clear<T: ManageTarget>(&mut self)
    where
        Self: Reposit<T>,
    {
        self.manager_mut().clear();
    }
    // -------
    #[inline]
    fn get<T: ManageTarget, const N: usize>(&mut self, label: [&str; N]) -> Option<[&'_ mut T; N]>
    where
        Self: Reposit<T>,
    {
        self.manager_mut().get_many_mut(label)
    }
    #[inline]
    fn get_one<T: ManageTarget>(&self, label: impl AsRef<str>) -> Option<&T>
    where
        Self: Reposit<T>,
    {
        self.manager().get(label.as_ref())
    }
    #[inline]
    fn get_one_mut<T: ManageTarget>(&mut self, label: impl AsRef<str>) -> Option<&mut T>
    where
        Self: Reposit<T>,
    {
        self.manager_mut().get_mut(label.as_ref())
    }
    // -------
    #[inline]
    fn remove<T: ManageTarget>(&mut self, label: impl AsRef<str>) -> Option<T>
    where
        Self: Reposit<T>,
    {
        self.manager_mut().remove(label.as_ref())
    }
    // -------
    #[inline]
    fn add<T: ManageTarget, const N: usize>(&mut self, ones: [T; N]) -> [&mut T; N]
    where
        Self: Reposit<T>,
    {
        let manager = self.manager_mut();

        let s = ones
            .into_iter()
            .map(|one| {
                let label = one.label().to_string();
                manager.insert(one.label().to_string(), one);
                label
            })
            .collect::<Vec<String>>();

        // TODO! optimization
        manager
            .get_many_mut(
                s.iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<&str>>()
                    .try_into()
                    .unwrap(),
            )
            .unwrap()
    }
    #[inline]
    /// Create and add a [`ManageTarget`] to the game. Use the `&mut Target` that is returned to adjust
    /// the translation, rotation, etc. Use a *unique* label for each sprite. Attempting to add two
    /// sprites with the same label will cause a crash.
    fn add_one<T: ManageTarget>(&mut self, one: T) -> &mut T
    where
        Self: Reposit<T>,
    {
        let manager = self.manager_mut();

        let label = one.label().to_string();
        manager.insert(label.to_string(), one);
        // Unwrap: Can't crash because we just inserted the sprite
        manager.get_mut(&label).unwrap()
    }
    // -------
    #[inline]
    #[allow(clippy::type_complexity)]
    fn drain_all<T: ManageTarget>(&mut self) -> Map<Drain<'_, String, T>, fn((String, T)) -> T>
    where
        Self: Reposit<T>,
    {
        self.manager_mut().drain().map(|(_, sprite)| sprite)
    }
    #[inline]
    fn drain<T: ManageTarget>(&mut self, mut predicate: impl FnMut(&&mut T) -> bool) -> Vec<T>
    where
        Self: Reposit<T>,
    {
        self.manager_mut()
            .drain_filter(|_, v| predicate(&v))
            .map(|(_, v)| v)
            .collect()
    }
    #[inline]
    fn drain_one<T: ManageTarget>(&mut self, predicate: impl FnMut(&&T) -> bool) -> Option<T>
    where
        Self: Reposit<T>,
    {
        let manager = self.manager_mut();
        if let Some(found) = manager.values().find(predicate) {
            return manager.remove(&found.label().to_owned());
        }
        None
    }
    // -------
    #[inline]
    fn find<T: ManageTarget>(&self, predicate: impl FnMut(&&T) -> bool) -> Vec<&T>
    where
        Self: Reposit<T>,
    {
        self.manager().values().filter(predicate).collect()
    }
    #[inline]
    fn find_one<T: ManageTarget>(&self, predicate: impl FnMut(&&T) -> bool) -> Option<&T>
    where
        Self: Reposit<T>,
    {
        self.manager().values().find(predicate)
    }
    // -------
    #[inline]
    fn delete<T: ManageTarget>(&mut self, predicate: impl FnMut(&&mut T) -> bool) -> bool
    where
        Self: Reposit<T>,
    {
        !self.drain(predicate).is_empty()
    }
    #[inline]
    fn delete_one<T: ManageTarget>(&mut self, predicate: impl FnMut(&&T) -> bool) -> bool
    where
        Self: Reposit<T>,
    {
        self.drain_one(predicate).is_some()
    }
    // -------
    #[inline]
    fn find_mut<T: ManageTarget>(&mut self, predicate: impl FnMut(&&mut T) -> bool) -> Vec<&mut T>
    where
        Self: Reposit<T>,
    {
        self.manager_mut().values_mut().filter(predicate).collect()
    }
    #[inline]
    fn find_one_mut<T: ManageTarget>(
        &mut self,
        predicate: impl FnMut(&&mut T) -> bool,
    ) -> Option<&mut T>
    where
        Self: Reposit<T>,
    {
        self.manager_mut().values_mut().find(predicate)
    }
    // -------
    #[inline]
    fn filter<T: ManageTarget, O: FnMut(&&T) -> bool>(
        &self,
        predicate: O,
    ) -> Filter<Values<'_, String, T>, O>
    where
        Self: Reposit<T>,
    {
        self.manager().values().filter(predicate)
    }
    #[inline]
    fn filter_mut<T: ManageTarget, O: FnMut(&&mut T) -> bool>(
        &mut self,
        predicate: O,
    ) -> Filter<ValuesMut<'_, String, T>, O>
    where
        Self: Reposit<T>,
    {
        self.manager_mut().values_mut().filter(predicate)
    }
    // -------
    #[inline]
    fn for_each<T: ManageTarget>(&self, f: impl FnMut(&T))
    where
        Self: Reposit<T>,
    {
        self.manager().values().for_each(f)
    }
    #[inline]
    fn for_each_mut<T: ManageTarget>(&mut self, f: impl FnMut(&mut T))
    where
        Self: Reposit<T>,
    {
        self.manager_mut().values_mut().for_each(f)
    }
    // -------
}
