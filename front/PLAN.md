# Back

- Declare `Action` struct
  - name: String
  - icon: Icon
    - path: String
  - action_type: enum ActionType
    - Open
      - path to program: String
      - args: String
    - GitPull
      -  git-dir: String

## Serialization

- Use serde_json for saving user config
- define `save_config()` function that gets called

# Front
 
- Also declare the same `Action` struct 
  - (any way to import rust struct into typescript?)
  
```svelte
export let actions: Action[] = [];
let edit: bool = false;

{#if edit}
  {#each actions as action, i}
    {name}
    <image href={image.load(icon)}/>
    {action_type}
  {/each}
{:else}
  {#each actions as action, i}
    <div contenteditable=true>{name}</div>
    <image href={image.load(icon)}/>
    <div contenteditatble=true>{action_type}</div>
  {/each}
{/if edit}
```

