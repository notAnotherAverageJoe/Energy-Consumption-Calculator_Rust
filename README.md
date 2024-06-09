# Energy Consumption Calculator 💡💰

This is a simple Rust program to calculate energy consumption and cost based on power consumption, running time, and cost per kWh.

## Usage ℹ️

To use this program, run it from the command line with the following arguments:

```sh
cargo run <power_consumption_kw> <running_time_hours> <cost_per_kwh>
```
## example code to run =  cargo run 
for power -> 2.5  for run time -> 8  for cost -> 0.12 
Replace <power_consumption_kw> with the power consumption in kilowatts, <running_time_hours> with the running time in hours, and <cost_per_kwh> with the cost per kilowatt-hour in USD.

Command-line Arguments 🛠️
<power_consumption_kw>: Power consumption in kilowatts.
<running_time_hours>: Running time in hours.
<cost_per_kwh>: Cost per kilowatt-hour in USD.


## When you run the program with the arguments 2.5 8 0.12, here's what happens:
## Output 📊
The program takes the power_consumption_kw (2.5 kW) and multiplies it by the running_time_hours (8 hours) to calculate the energy consumption:

Energy Consumption = 2.5 kW * 8 hours = 20 kWh
Then, it takes the calculated energy consumption (20 kWh) and multiplies it by the cost_per_kwh ($0.12) to calculate the cost:

Cost = 20 kWh * $0.12/kWh = $2.40
So, the program outputs:

```yaml
Energy Consumption: 20.00 kWh
Cost: $2.40
This means that consuming 2.5 kW of power for 8 hours will result in an energy consumption of 20.00 kWh and will cost $2.40 USD at a rate of $0.12 per kWh.
```