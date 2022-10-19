# Distributed Stream Processor

Using:
- Rust as the primary language
- WASM as the IR (wire transfer, coordination)
- Arrow as the intermediate data representation

There is a pool of N coordinators and M workers. Coordinator nodes are sync'd using RAFT. Clients
propose new plans to the coordinator cluster, and if accepted the coordinator cluster tells workers
to download the plan executable, finish processing their current batch of input events, wait to 
start, and then begin processing new batches of events. 


Parameters:
- Global wall clock 
- Global logical clock (happens before?)
- Batch size
- Delay before accepting next batch (e.g., coordination period)

Design Goals:
- Minimize start up time before accepting new ensemble
- Minimize event-delivery jitter (i.e., delta between actual event time and expected event time)
