//! Gestion du bargraph et mise à jour en fonction de la valeur partagée

use crate::bargraph::Bargraph;
use embassy_executor::task;
use embassy_stm32::gpio::Output;
#[task]
pub async fn bargraph_task(mut bargraph: Bargraph<[Output<'static>; 8], Output<'static>>) {
    loop {
        let _ = bargraph.wait_and_update().await;
    }
}