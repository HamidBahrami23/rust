/* 
## 🦀 Mini Project Exercise: **Simple Task Manager (CLI logic only)**

You are going to build the **core logic** of a very small task manager (like a TODO app), **without UI** and **without file storage**.

Think of it as the “engine” of a real app.

---

## 🎯 Goal of the Exercise

Manage tasks that:

* Have a title
* Have a status (Todo / InProgress / Done)
* Can be searched by id
* Can be updated safely

You will practice **how Rust forces you to handle all cases correctly**.

---

## 📦 Part 1: Define Your Data (👉 `enum`)

### Task Status

**Exercise**
Create an `enum` called `TaskStatus` with **three variants**:

* `Todo`
* `InProgress`
* `Done`

📌 **Why enum here?**

* A task can only be in **one of these exact states**
* Rust should prevent invalid states

💡 Tip:

> Ask yourself: *Why not use `String`?*
> Because enums give compile-time safety.*/

enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

/*
---

## 🧱 Part 2: Create a Struct and Implement Methods (👉 `struct` + `impl`)

### Task Structure

**Exercise**
Create a `struct` called `Task` with:

* `id: u32`
* `title: String`
* `status: TaskStatus`

Then write an `impl Task` block.

Inside `impl`, define:

1. A function to **create a new task**
2. A function to **change the task status**

📌 **Why `impl`?**

* These functions *belong to Task*
* This is how real Rust projects are structured

💡 Tip:

> Constructor-like functions are usually called `new`
*/
struct Task {
    id: u32,
    title: String,
    status: TaskStatus,
}

impl Task {
    fn new() -> Self {
        Self {}
    }
    fn task_status() {

    }
}

/*---

## 🔁 Part 3: Store Tasks (👉 `Vec<Task>` + functions)

You will manage tasks using a `Vec<Task>`.

**Exercise**
Write a function (NOT inside `impl Task`) that:

* Takes a `&Vec<Task>`
* Takes a task `id`
* Returns the task **if found**

📌 **Important**
The function must return:

```rust
Option<&Task>
```

📌 **Why `Option` here?**

* The task **may or may not exist**
* Rust forces you to handle the “not found” case

💡 Tip:

> Any time something *can fail but is not an error*, `Option` is your friend.

*/

fn store(a: &Vec<Task> , b: Task) -> Option<&Task> {}



/*---

## 🎯 Part 4: Handle Results Safely (👉 `match`)

**Exercise**
Write a function that:

* Calls the “find task by id” function
* Uses `match` to:

  * Print task info if found
  * Print a clear message if not found

📌 **Why `match`?**

* You must handle **all possible cases**
* Rust won’t let you forget `None`

💡 Tip:

> Don’t use `if let` yet — this exercise is about `match`.

*/

fn find_task(a:) {
    match a {

    }
}


/*---

## 🔄 Part 5: Update Task Status (👉 `match` + `enum`)

**Exercise**
Write a function that:

* Takes `&mut Vec<Task>`
* Takes a task `id`
* Takes a new `TaskStatus`
* Updates the task if it exists

Use:

* `Option`
* `match`
* `enum` variants

📌 **Why all three together?**

* `Option` → task may not exist
* `enum` → limited valid statuses
* `match` → safe branching logic

💡 Tip:

> You will probably need mutable references here — that’s intentional 😄

---

## 🧪 Part 6: Simulate Real Usage (👉 functions working together)

**Exercise**
In `main()`:

1. Create an empty task list
2. Add 2–3 tasks
3. Try:

   * Printing an existing task
   * Printing a non-existing task
   * Updating a task’s status
   * Updating a task that doesn’t exist

📌 **Why this matters**
This mimics:

* User input
* Edge cases
* Real application flow
*/

fn main() {
    let task_list = vec![
        
    ]
}



/*---


## 🧠 Final Reflection Questions (VERY IMPORTANT)

After finishing, ask yourself:

1. Where did Rust **force** me to think more?
2. Which bugs are now **impossible**?
3. How did `enum + match` reduce runtime errors?
4. Why is `Option` better than returning `-1` or `null`?

---

## 🚀 Next Step (optional)

When you’re done, a **natural next exercise** would be:

* Replace `print` with returning `Result`
* Add `Error` enum
* Store tasks in a file (later)

---

If you want, next time I can:

* Review *your solution*
* Or rewrite this same project at **slightly higher difficulty**
* Or map each part to **real production Rust code**

You’re learning Rust the *right* way 👏

*/