<script lang="ts">
    import { onMount } from 'svelte';
    import { ZoomIn, ZoomOut, Maximize2, ArrowUp } from 'lucide-svelte';
    import { dragState } from './stores/DiagramStores.js';
    import type { BlockData } from '../models/blockData';

    export let blocks: Record<string, BlockData>;
    export let currentBlockId: string;
    let wrapper: HTMLDivElement;
    let container: HTMLDivElement;
    let scale = 1;
    let isDragging = false;
    let startX = 0;
    let startY = 0;
    let translateX = 0;
    let translateY = 0;
    let parentStack: string[] = [];

    // Drag threshold (in pixels)
    const DRAG_THRESHOLD = 5;
    let isMouseDown = false;
    let initialMouseX = 0;
    let initialMouseY = 0;

    $: {
        parentStack = [];
        let currentBlock = blocks[currentBlockId];
        while (currentBlock?.caller) {
            parentStack.unshift(currentBlock.caller);
            currentBlock = blocks[currentBlock.caller];
        }
    }

    function handleWheel(event: WheelEvent) {
        if (event.ctrlKey) {
            event.preventDefault();
            const delta = event.deltaY > 0 ? -0.1 : 0.1;
            updateZoom(scale + delta);
        }
    }

    function updateZoom(newScale: number) {
        scale = Math.min(Math.max(0.1, newScale), 3);
        updateTransform();
    }

    function startDrag(event: MouseEvent) {
        if (event.button === 0) { // Left mouse button
            isMouseDown = true;
            initialMouseX = event.clientX;
            initialMouseY = event.clientY;
            startX = event.clientX - translateX;
            startY = event.clientY - translateY;
            wrapper?.style.setProperty('cursor', 'grab');
        }
    }

    function drag(event: MouseEvent) {
        if (isMouseDown && !isDragging) {
            // Check if the mouse has moved beyond the threshold
            const dx = event.clientX - initialMouseX;
            const dy = event.clientY - initialMouseY;
            if (Math.sqrt(dx * dx + dy * dy) >= DRAG_THRESHOLD) {
                isDragging = true;
                dragState.set({ isDragging: true });
                wrapper?.style.setProperty('cursor', 'grabbing');
            }
        }

        if (isDragging) {
            translateX = event.clientX - startX;
            translateY = event.clientY - startY;
            updateTransform();
        }
    }

    function stopDrag() {
        if (isDragging) {
            isDragging = false;
            dragState.set({ isDragging: false });
        }
        isMouseDown = false;
        wrapper?.style.setProperty('cursor', 'grab');
    }

    function updateTransform() {
        if (container) {
            container.style.willChange = 'transform';
            container.style.transform = `translate3d(${translateX}px, ${translateY}px, 0) scale(${scale})`;

            setTimeout(() => {
                container.style.willChange = 'auto';
            }, 200);
        }
    }

    function fitToScreen() {
        if (wrapper && container) {
            const wrapperRect = wrapper.getBoundingClientRect();
            const containerRect = container.getBoundingClientRect();

            const scaleX = wrapperRect.width / (containerRect.width / scale);
            const scaleY = wrapperRect.height / (containerRect.height / scale);
            scale = Math.min(scaleX, scaleY, 1) * 0.9;

            translateX = (wrapperRect.width - (containerRect.width * scale)) / 2;
            translateY = (wrapperRect.height - (containerRect.height * scale)) / 2;

            updateTransform();
        }
    }

    onMount(() => {
        fitToScreen();
    });
</script>

<div
        class="diagram-wrapper relative w-full h-full bg-gray-900 overflow-hidden"
        bind:this={wrapper}
        on:wheel={handleWheel}
        on:mousedown={startDrag}
        on:mousemove={drag}
        on:mouseup={stopDrag}
        on:mouseleave={stopDrag}
>
    <div class="absolute top-4 right-4 flex gap-2 z-10">
        <button
                class="p-2 bg-gray-800 rounded-lg hover:bg-gray-700 transition-colors"
                on:click={() => updateZoom(scale + 0.1)}
        >
            <ZoomIn class="w-5 h-5" />
        </button>
        <button
                class="p-2 bg-gray-800 rounded-lg hover:bg-gray-700 transition-colors"
                on:click={() => updateZoom(scale - 0.1)}
        >
            <ZoomOut class="w-5 h-5" />
        </button>
        <button
                class="p-2 bg-gray-800 rounded-lg hover:bg-gray-700 transition-colors"
                on:click={fitToScreen}
        >
            <Maximize2 class="w-5 h-5" />
        </button>
    </div>

    {#if parentStack.length > 0}
        <div class="absolute top-4 left-4 z-10">
            <button
                    class="flex items-center gap-2 p-2 bg-gray-800 rounded-lg hover:bg-gray-700 transition-colors"
                    on:click={() => dispatch('navigate', parentStack[parentStack.length - 1])}
            >
                <ArrowUp class="w-5 h-5" />
                <span>Go to Parent</span>
            </button>
        </div>
    {/if}

    <div class="absolute bottom-4 right-4 text-sm text-gray-400">
        {Math.round(scale * 100)}%
    </div>

    <div
            class="diagram-container"
            bind:this={container}
            style="transform: translate3d({translateX}px, {translateY}px, 0) scale({scale})"
    >
        <slot />
    </div>
</div>