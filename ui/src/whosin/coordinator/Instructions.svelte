<script>
    import { navigate } from '../../store.js';

    async function goToCreate() {
      navigate("create-coordination", {});
    }

    async function gotToAllCoordinations() {
      navigate("all-coordinations", {});
    }
</script>

<div class="instructions">

<div class="content">
    <div style="margin-left:auto;margin-right:auto;width:600px;">
        <h1>Welcome to <strong>Who's In?</strong></h1>
        <h3>Collaborate with strangers or friends.</h3>
        <!-- <h3>Work together with less risk.</h3> -->
    
    <!-- <h3>Plan get-togethers, events or actions of any kind.</h3> -->
    
    
    <h2>How it works</h2>
    <ol>
      <li>Think of an action or event</li>
      <li>Explain all necessary roles</li>
      <li>Invite people to fill those roles</li>
      <li>Don't act unless all roles are filled</li>
    </ol>

    <p>Get started and <button class="gotoButton" on:click={() => gotToAllCoordinations()}>View public coordinations</button> or <button class="gotoButton" on:click={() => goToCreate()}>create a new agreement</button>.</p>
    </div>
</div>
</div>

<style>
    .instructions {
        height: 100vh;
        width: 100%;
        top: 0;
        z-index: 0;
        position: absolute;
        background: url("../../assets/whosin-background.jpg");
        background-size: cover;
        /* slow fade in background */
        -webkit-animation: fadein 1s; /* Safari, Chrome and Opera > 12.1 */
        -moz-animation: fadein 1s; /* Firefox < 16 */
        -ms-animation: fadein 1s; /* Internet Explorer */
        -o-animation: fadein 1s; /* Opera < 12.1 */
        animation: fadein 1s;
    }
    .instructions .content {
        /* margin: 0 auto;
        max-width: 800px;
        padding: 1em; */
        /* margin-left:auto;
        margin-right:auto;
        width:300px; */

        margin-top: 200px;
        width:100%; 
        position:relative;
        z-index:1;
        text-align: left;
        color: white;
    }

    .instructions li {
        /* list-style: none; */
        margin-top: 0.2em;
        /* margin-left: 20px; */
    }
    
    .instructions h1 {
        font-size: 3em;
        font-weight: 700;
        margin: 0;
        padding: 0;
    }

    .instructions h1 strong {
        /* color: #f5a623; */
        color: #1952bb;
    }
    
    .instructions h2 {
        font-size: 2em;
        font-weight: 700;
        /* margin: 0; */
        margin-top: 24px;
        padding: 0;
        color: white;
    }
    
    .instructions h3 {
        font-size: 1.4em;
        font-weight: 700;
        /* margin: 0; */
        margin-top: 10px;
        padding: 0;
    }
    
    .instructions ol {
        margin-top: 20px;
    }
    
    .instructions li {
        font-size: 1.2em;
        /* margin: 0; */
        /* padding: 0; */
    }
    
    /* .instructions .content {
        margin: 0;
        padding: 0;
        margin-top: 200px;
    }
    
    .instructions article p {
        margin: 0;
        padding: 0;
    } */

    .gotoButton {
        border: 0;
        background: #1952bb;
        padding: 6px;
        border-radius: 4px;
        color: white;
        cursor: pointer;
    }

    .gotoButton:hover {
        background: #457de3;
    }
</style>