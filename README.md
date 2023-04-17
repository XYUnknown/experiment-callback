# Experiment - A Callback Reminder
This repository is prepared for evaluating the effort to refactor an application that runs on a single nodes into an application that runs on multiple nodes.

## Getting Started
- Forking the repository.
- Cloning the forked repository into your own machine.
- Running all tests: `cargo test`
    - You should see the following output:
    ```
    test reminder::ready_reminder_server::tests::cmp_works ... ok
    test reminder::ready_reminder_server::tests::lt_works ... ok
    ```
- Executing the provided single node application: `cargo run --example ready_reminder_callback_client_single`
    - You should see the following output:
    ```
    Task Hello World! is due
    Task Goodbye World! is due
    ```

## Explanation of the Provided Template Code
- `./experiment_callback/src/reminder/ready_reminder_callback_server.rs` contains:
    - the implementation of an event entry (`Entry`) that will become ready at some time in the future.
    - the implementation of a server (`ReadyReminderServer`) that stores events that will become ready at some time in the future. Events can be submitted to the server, and the server will send notification (as callbacks) when events are ready.
- `./experiment_callback/src/reminder/callback.rs` contains:
    - the implementation of an callback function (`CallBack`) which can be passed as a part of an event entry, and get called when the event is ready.
- `./experiment_callback/examples/ready_reminder_callback_client_single.rs` is an application that runs on a single node, which specifies times at which some events will become ready, then get notified when those events are ready.

## Task
- Please refactor this application to run on multiple nodes. The `ReadyReminderServer` should run on a node, accepting requests from client(s) nodes that submit events. It will notify clients when events are ready.
- Feel free to choose any framework for refactoring this application.

## Evaluation

Please work independently of the other participants. 

Please keep track of the amount of time that you spend on various programming tasks (including but not limited to reading documentations, seeking support, design, implementation, testing, and debugging). For each task please record observations if pertinent (especially levels of satisfaction or frustration with the process).
