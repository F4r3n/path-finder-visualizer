<script lang="ts">

import { onMount } from 'svelte';
import * as Grid from "./types/grid"
export let sizeX : number;
export let sizeY : number;


let currentIndexAnimation = 0;
let currentIndexAnimationPath = 0;
export const clear = () => listCurrentPosition = Array(sizeX*sizeY).fill(0);
export const setAction = (inAction : number) => actionKind = inAction;
export const launchAnimation = (inArray : Array<number>, inPathFound : Array<number>) => {
    let frame;
    currentIndexAnimation = 0;
    currentIndexAnimationPath = 0;
    function loop() {
        if(currentIndexAnimation >= inArray.length - 1)
            return;


        frame = requestAnimationFrame(loop);
        for(let i = 0; i < listCurrentPosition.length/100; ++i)
        {
            const index = inArray[currentIndexAnimation];
            if(listCurrentPosition[index] === Grid.SQUARE_KIND.path.value)
            {
                currentIndexAnimation++;
                continue;
            }
                
            listCurrentPosition[index] = Grid.SQUARE_KIND.done.value;

            currentIndexAnimation++;
        }


        for(let i = 0; i < listCurrentPosition.length/1000; ++i)
        {
            if(currentIndexAnimationPath >= inPathFound.length - 1)
                break;
            listCurrentPosition[inPathFound[currentIndexAnimationPath]] = Grid.SQUARE_KIND.path.value;

            currentIndexAnimationPath++;
        }
        listCurrentPosition = listCurrentPosition;

    }

    loop();

    return () => cancelAnimationFrame(frame);
}
let actionKind = 0;
let mouseDown = false;
let listCurrentPosition : Array<number> = []
onMount(async () => {
    listCurrentPosition = Array(sizeX*sizeY).fill(0);
});
export let listValues : Array<number> = []

</script>

<main>
    <div draggable="false" on:mousedown={()=> {mouseDown = true;}} on:mouseup={()=> {mouseDown = false;}} 
    style="--sizeX:{sizeX}; --sizeY:{sizeY}" class="container">
        {#each listValues as value, i}
            <div draggable="false"
                class="square"
                class:arrival={value === Grid.SQUARE_KIND.arrival.value}
                class:departure={value === Grid.SQUARE_KIND.departure.value}
                class:obstacle={value === Grid.SQUARE_KIND.obstacle.value}
                


                on:mousedown={()=> {
                    listValues[i] = actionKind;
                    listCurrentPosition[i] = 0;
                }}
                on:focus={()=> {}}
                on:mouseover={(e)=> {
                    e.preventDefault()
                    if(mouseDown)
                    {
                        listValues[i] = actionKind;
                        listCurrentPosition[i] = 0;
                    }
                }}>
                {#if listCurrentPosition[i] === Grid.SQUARE_KIND.done.value || listCurrentPosition[i] === Grid.SQUARE_KIND.path.value}
                    <div class="layer" class:path={listCurrentPosition[i] === Grid.SQUARE_KIND.path.value}></div>
                {/if}
            </div>
        {/each}
    </div>
</main>


<style>

    main {
        display: flex;
        flex-direction: row;
    }   
    .options {
        display: flex;
        flex-direction: column;
    }

    .container {
        display: grid;
        grid-template: repeat(var(--sizeX), calc(500px/var(--sizeX)))/ repeat(var(--sizeY), calc(500px/var(--sizeY)));
        grid-gap: 1px;
        background-color: black;
        width: fit-content;
        height: fit-content;
        border:1px solid ;
        border-color: black;

    }

    .container div {
        aspect-ratio: 1;

    }

    .square {
        background-color: beige;
    }

    .obstacle {
        background-color: black;
    }

    .layer {
        background-color: rgba(248, 0, 0, 0.5);
        width: 100%;
        height: 100%;
    }

    .arrival {
        background-color: green;
    }

    .departure {
        background-color: blue;
    }

    .path {
        background-color: rgb(0, 174, 255);
    }

</style>