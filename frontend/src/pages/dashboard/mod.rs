pub mod loading;
pub mod memory;
pub mod disk;
pub mod call_log;
pub mod extension;
pub mod trunk;
pub mod os;

use yew::prelude::*;

use memory::{MemoryCard};
use loading::{LoadingCard};
use disk::{DiskCard};
use call_log::{CallLogCard};
use extension::{ExtenCard};
use trunk::{TrunkCard};

#[function_component]
pub fn Dashboard() -> Html {
    html! {
        <div>
            <div class="flex flex-wrap pbx-card justify-center m-2">
                <MemoryCard />
                <LoadingCard />
                <DiskCard />
            </div>
            <div class="flex flex-wrap pbx-card justify-center m-2">
                <ExtenCard />
            </div>
            <div class="flex flex-wrap pbx-card justify-center m-2">
                <TrunkCard />
            </div>
            <div class="flex flex-wrap pbx-card justify-center m-2">
                <CallLogCard />
            </div>
        </div>
    }
}
