<script lang="ts">
  import { onMount } from 'svelte';
  import Grid from './Grid.svelte';
  import * as GridType from "./types/grid"
  import { invoke } from '@tauri-apps/api/tauri'
 
let sizeX : number = 50;
let sizeY : number = 50;
onMount(async () => {
    for(let i = 0; i < sizeX*sizeY; ++i)
        list.push(0);

    list[0] = GridType.SQUARE_KIND.departure.value;
    list[list.length - 1] = GridType.SQUARE_KIND.arrival.value;

    list = list;

});
let gridComponent;
let list : Array<number> = [];


</script>

<div class="column margin-left">

<div class="row">

  <div class="row">

  <Grid bind:this={gridComponent} {sizeX} {sizeY} listValues={list}></Grid>
  <div class="column space margin-left">

    <div class="column space grow">
      <button on:click={()=> {gridComponent.setAction(GridType.SQUARE_KIND.arrival.value)}}>{GridType.SQUARE_KIND.arrival.text}</button>
      <button on:click={()=> {gridComponent.setAction(GridType.SQUARE_KIND.departure.value)}}>{GridType.SQUARE_KIND.departure.text}</button>
      <button on:click={()=> {gridComponent.setAction(GridType.SQUARE_KIND.obstacle.value)}}>{GridType.SQUARE_KIND.obstacle.text}</button>
      <button on:click={()=> {gridComponent.setAction(GridType.SQUARE_KIND.clear.value)}}>{GridType.SQUARE_KIND.clear.text}</button>
    </div>


    <button class="important" on:click={()=> {
      gridComponent.clear();
      invoke("find_path",{info:{list:list, size_x:sizeX, size_y:sizeY}}).then(result => {
        console.log(result)
        gridComponent.launchAnimation(result.list, result.path)
      })
  
      }}>Start</button>
  </div>


  </div>
</div>


</div>

<style>
  .column {
    display: flex;
    flex-direction: column;
  }

  .row {
    display: flex;
    flex-direction: row;
  }

  .space {
    justify-content: space-between;
  }

  .grow {
    flex: 0.25 0 0;
  }

  .important {
    background-color: rgb(187, 36, 36);
  }

  .margin-left {
    margin-left: 5px;
  }

</style>
