//! Gestion du bargraph et mise à jour en fonction de la valeur partagée

use crate::bargraph::Bargraph;
use embassy_executor::task;
#[task]
pub async fn bargraph_task(mut bargraph: Bargraph<8>) {
    loop {
        let _ = bargraph.wait_and_update().await;
    }
}