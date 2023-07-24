# features
- add a item
    - cmd: rtd -a(--add) <item-name>
    - add a item to list, default uncompleted, undeleted, gen a id
- delete a item
    - cmd: rtd -d(--delete) <item-id>
    - mark a item to deleted
- complete a item
    - cmd: rtd -c(--complete) <item-id>
- uncomplete a item
    - cmd: rtd -u(--uncomplete) <item-id>
- list all item
    - cmd: rtd -l all(--list all)
    - list all items(completed,uncompleted,deleted)
- list all uncompleted item
    - cmd: rtd -l(--list)
    - list all uncompleted items
- list all completed item
    - cmd: rtd -l completed(--list completed)
    - list all completed items
- list all deleted item
    - cmd: rtd -l deleted(--list deleted)
    - list all deleted items
- real delete
    - cmd: rtd --real-delete
    - real delete all mark to deleted item
- clear
    - cmd: rtd --clear
    - clear all items, make list empty

# storage
save to a `/${HOME}/.rtd.csv` file

|id|name|completed|deleted|createdAt|completedAt|deletedAt|
|-|-|-|-|-|-|-|
|1|homework|false|false|234234234|234234234|234234234|
|2|todo item|false|false|234234234|234234234|234234234|
