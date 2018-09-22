# drat

Daily routine automated tracker - as in "drat, I forgot to do my life right because I don't use drat".  It's a glorified to-do list.

As much as I want to do this with ReasonReact, FORCE yourself do do it with the JS ecosystem.

Copy your code somewhere else and then build up your react app without Create React App!

Kind of like Outlook flags - they can recur with granular options, but they can also hold data about the activity.

* One off single tasks - just a checkbox
* Recurring activity - exercise, musical insturment practice - can carry duration of activity - in these cases the button click starts the timer, click again to stop and confirm the time to log
* Programming work - measuing what?  time?  commits?
* Medication/supplement reminders - carry recurrance, dose - maybe even reminders about re-ordering?

The frontend just recieves the daily single instance versions - all recurrence is handled in the backend

Theres a difference between a Task and a TaskInstance - a Task carries the reccurrance/history stuff, and a TaskInstance is dateless - it's a one-off generated from the Task that doesnt know its own history.  Even if a Task is just a one-off, the date will stay with the Task in the backend, while will send up the proper TaskInstance for the user to click in the frontend.  These are simple, just a boolean for "completed", so make these work first. Don't worry about assigning a specific user, this is a single-user app until I get it working.

Backend:

Stores -> Task (id, title, completed, date)
Responds -> TaskInstance (id, title)

Receives -> TaskInstance (id, title)
Renders -> TaskInstance (title, completed) - when completed, send a request to toggle the backend Task to Completed

CRUD:

C:
Frontend hits GET /task/new/{title}
Backend creates Task, returns TaskInstance

R - there will be a "Read All of Today's Tasks" which will return all the taskinstances, as well as "Read Single Task", which I'm not sure will really be used?  Maybe to refresh a single task?:
Frontend hits GET /task/{id}
Backend looks up Task, returns TaskInstance

U:
Frontend hits PUT /task/{id} with JSON body of new data
Backend adjusts the proper Task, returns newly generated TaskInstance

D:
Frontend hits GET /task/delete/{id}
Backend removes the Task entirely, returns HttpResponse::Ok()

There will also be separated endpoints for looking into Task history, beyond just a daily TaskInstance
