
use yew::prelude::*;

#[function_component]
pub fn DashboardCard() -> Html {
    let r = 1..8;
    html!{
        <div class="overflow-x-auto">
            <table class="table table-xs">
                <thead>
                    <tr>
                        <th></th>
                        <th>{"Name"}</th>
                        <th>{"Job"}</th>
                        <th>{"location"}</th>
                        <th>{"Favorite Color"}</th>
                    </tr>
                </thead>
                <tbody>                      
                {
                r.into_iter().map(|r|{
                    html!{
                        <tr>
                            <th>{r}</th>
                            <td>{"Cy Ganderton"}</td>
                            <td>{"Canada"}</td>
                            <td>{"12/16/2020"}</td>
                            <td>{"Blue"}</td>
                        </tr>}
                }).collect::<Html>()                       
                }
            
                </tbody>
            </table>
        </div>
    }

}
#[function_component]
pub fn Dashboard() -> Html {
    html! {
            <div class="flex flex-wrap">
                <div class="card w-1/2 bg-base-100 shadow-xl">
                    <div class="card-body">
                    <h2 class="card-title">{"User"}</h2>
                        <DashboardCard />
                        <div class="card-actions justify-end">
                            <div class="badge badge-outline">{"User"}</div>
                        </div>
                    </div>
                </div>

                <div class="card w-1/2 bg-base-100 shadow-xl">
                    <div class="card-body">
                        <h2 class="card-title">
                            {"Active Calls"}
                        </h2>
                        <DashboardCard />
                    </div>
                </div>

                <div class="card w-1/2 bg-base-100 shadow-xl">
                    <div class="card-body">
                        <h2 class="card-title">
                            {"Recent calls"}
                        </h2>
                        <DashboardCard />
                        <div class="card-actions justify-end">
                            <div class="badge badge-outline">{"CDR"}</div>
                        </div>
                    </div>
                </div>

                <div class="card w-1/2 bg-base-100 shadow-xl">
                    <div class="card-body">
                        <h2 class="card-title">
                            {"System"}
                        </h2>
                        <DashboardCard />
                    </div>
                </div>
            </div>
        }
}
