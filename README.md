# drat

Daily routine automated tracker - as in "drat, I forgot to do my life right because I don't use drat".  

It's a to-do list.

Backend in actix_web with diesel, frontend in React.js with MobX.

## Requirements

* Stable rust
* yarn
* PostgreSQL
* `diesel-cli` -- `cargo install diesel-cli`

## Usage

```shell
git clone https://github.com/deciduously/drat
cd drat
echo "DATABASE_URL=postgres://username:password@localhost/drat" > .env
diesel setup
yarn
yarn start
```

`yarn start` will run everything in one terminal, you can use `yarn watch:rs` and `yarn watch:js` in separate terminals to keep frontend and backend errors separate.

## Todo 

Kind of like Outlook flags - they can recur with granular options, but they can also hold data about the activity.

* One off single tasks - just a checkbox
* Recurring activity - exercise, musical insturment practice - can carry duration of activity - in these cases the button click starts the timer, click again to stop and confirm the time to log
* Programming work - measuing what?  time?  commits?
* Medication/supplement reminders - carry Recurrence, dose - maybe even reminders about re-ordering?

