use std::{any::{Any, TypeId}, collections::HashMap, ops::{Deref, DerefMut}, sync::Arc};

use tokio::sync::{Notify, RwLock, RwLockReadGuard, RwLockWriteGuard};


pub trait DiscordData: Send + Sync + 'static {}

struct NotifyWriteGuard<'a, T> {
    guard: RwLockWriteGuard<'a, T>,
    notify: Arc<Notify>
}

impl<'a, T> Deref for NotifyWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.guard
    }
}

impl<'a, T> DerefMut for NotifyWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.guard
    }
}

impl<'a, T> Drop for NotifyWriteGuard<'a, T> {
    fn drop(&mut self) {
        self.notify.notify_waiters();
    }
}

struct ObservableField<T> {
    value: Arc<RwLock<T>>,
    notify: Arc<Notify>
}


pub struct ClientDataStore {
    fields: HashMap<TypeId, Arc<dyn Any + Send + Sync + 'static>>,
}

pub struct StoreError {}

impl ClientDataStore {

    fn get_field<T: DiscordData>(&self) -> Option<&ObservableField<T>> {
        let field = self.fields.get(&TypeId::of::<T>())?;
        let field = field.downcast_ref::<ObservableField<T>>()?;

        Some(field)
    }

    pub fn insert<T: DiscordData>(&mut self, value: T) {
        let field = ObservableField {
            value: Arc::new(RwLock::new(value)),
            notify: Arc::new(Notify::new()),
        };

        self.fields.insert(
            TypeId::of::<T>(), 
            Arc::new(field) as Arc<dyn Any + Send + Sync>
        );
    }

    pub fn delete<T: DiscordData>(&mut self) -> Result<(), StoreError> {
        self.fields.remove(&TypeId::of::<T>()).ok_or(StoreError {})?;
        Ok(())
    }

    pub async fn write<T: DiscordData>(&self) -> Option<NotifyWriteGuard<'_, T>> {
        let field = self.get_field::<T>()?;

        let guard = field.value.write().await;

        Some(NotifyWriteGuard { 
            guard, 
            notify: field.notify.clone(),
        })
    }

    pub async  fn read<T: DiscordData>(&self) -> Option<RwLockReadGuard<'_, T>> {
        let field = self.get_field::<T>()?;

        Some(field.value.read().await)
    }

    pub async fn wait_for_update<T: DiscordData>(&self) -> Result<(), StoreError> {
        let field = self.get_field::<T>().ok_or(StoreError {})?;

        field.notify.notified().await;

        Ok(())
    }
}
