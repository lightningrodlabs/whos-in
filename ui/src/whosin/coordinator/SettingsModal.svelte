<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import { setBackgroundImage, loadState, backgroundImage } from '../../crud/localStorage';
    import { onMount } from 'svelte';

    export let showSettingsModal: boolean;
    let customImage: boolean = false;
    const dispatch = createEventDispatcher();

    // function handleImageChange(event) {
    //     const file = event.target.files[0];
    //     if (file) {
    //         const reader = new FileReader();
    //         reader.onload = (e) => {
    //             backgroundImage = e.target.result;
    //             dispatch('changeBackground', { backgroundImage });
    //         };
    //         reader.readAsDataURL(file);
    //     }
    // }

    function closeModal() {
        showSettingsModal = false;
    }
</script>

<div class="modal">
    <div class="modal-header">
        <h2>Theme</h2>
        <button on:click={closeModal}>X</button>
    </div>
    <div class="modal-body">
        <!-- <h3>Background</h3> -->
        <select
            value={$backgroundImage}
            on:change={(e) => {
                customImage = e.target.value === 'custom';
                if (!customImage) {
                    setBackgroundImage(e.target.value);
                }
                dispatch('changeBackground', { backgroundImage });
            }}
        >
            <option value="none">Basic</option>
            <option value="/src/assets/backgrounds/woods.jpeg">Woods</option>
            <option value="/src/assets/backgrounds/mountain.jpeg">Mountain</option>
            <option value="/src/assets/backgrounds/texture.jpg">Texture</option>
            <option value="/src/assets/backgrounds/birds.jpg">Birds</option>
            <option value="/src/assets/backgrounds/abstract.jpg">Abstract</option>
            <option value="/src/assets/backgrounds/misty.jpg">Misty</option>
            <option value="/src/assets/backgrounds/pond.jpg">Pond</option>
            <option value="/src/assets/backgrounds/sheep.jpg">Sheep</option>
            <option value="custom">Custom URL</option>
        </select>
        <!-- if custom, reveal text input -->
        {#if customImage}
            <input type="text" id="background-image" accept="image/*" 
            on:change={(e) => {
                setBackgroundImage(e.target.value);
            }} />
        {/if}
        <!-- <input type="file" id="background-image" accept="image/*" on:change={handleImageChange} /> -->
    </div>
</div>

<div class="full-screen-background" on:click={closeModal}>
</div>

<style>
    /* .modal {
        position: fixed;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        background: white;
        padding: 20px;
        border-radius: 8px;
        box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
    } */
    .modal {
        padding: 10px;
        position: relative;
        z-index: 2;
        background-color: transparent;
        width: 100%;
        cursor: auto;
    }
    .modal-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    .modal-body {
        margin-top: 10px;
    }
    h2 {
        font-size: 1.5rem;
        text-align: center;
        width: 100%;
        margin-top: 0;
        margin-bottom: 0;
    }

    .full-screen-background {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0);
        display: flex;
        justify-content: center;
        align-items: center;
        cursor: auto;
    }

    input[type="text"] {
        width: 100%;
        margin-top: 10px;
    }
</style>