use core::sync::atomic::AtomicU32;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

//Variable partagées pour :

//Pargraph
pub static BARGRAPH_LEVEL: AtomicU32 = AtomicU32::new(0);
pub static BARGRAPH_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

