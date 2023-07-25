# features
- add a item
    - cmd: rtd -a(--add) <item-name>
    - add a item to list, default uncompleted, undeleted, gen a id
- complete a item
    - cmd: rtd -c(--complete) <item-id>
- uncomplete a item
    - cmd: rtd -u(--uncomplete) <item-id>
- delete a item
    - cmd: rtd -d(--delete) <item-id>
    - mark a item to deleted
- restore a deleted item
    - cmd: rtd -r(--restore) <item-id>
    - restore a deleted marked item
- destroy
    - cmd: rtd --desroty <item-id>
    - destroy a item
- destroy deleted
    - cmd: rtd --destroy-deleted
    - destroy all deleted item
- clear
    - cmd: rtd --clear
    - clear all items, make list empty
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

# storage
save to a `/${HOME}/.rtd.csv` file

|id|name|completed|deleted|createdAt|completedAt|deletedAt|
|-|-|-|-|-|-|-|
|1|homework|false|false|234234234|234234234|234234234|
|2|todo item|false|false|234234234|234234234|234234234|
