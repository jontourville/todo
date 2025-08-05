# todo - CLI TODO list

todo is a simple CLI TODO list manager that provides a list of tasks and
due dates.

## Examples

```bash
# Add a new task
todo add "Pick up groceries"

# Add a new task with a due date
todo add "Vet appointment" 2025-09-01

# Print the TODO list
todo

# Print the TODO list ordered by due date
todo date

# Remove the second task
todo remove 2

# Move a task from position 1 to 2
todo move 1 2
```
