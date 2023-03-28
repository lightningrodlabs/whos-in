import { assert, test } from "vitest";

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource,  fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createCoordination } from './common.js';

test('link a Coordination to a SpamReporter', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/dcan.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const baseRecord = await createCoordination(alice.cells[0]);
    const baseAddress = baseRecord.signed_action.hashed.hash;
    const targetAddress = alice.agentPubKey;

    // Bob gets the links, should be empty
    let linksOutput: Record[] = await bob.cells[0].callZome({
      zome_name: "coordinator",
      fn_name: "get_spam_reporters_for_coordination",
      payload: baseAddress
    });
    assert.equal(linksOutput.length, 0);

    // Alice creates a link from Coordination to SpamReporter
    await alice.cells[0].callZome({
      zome_name: "coordinator",
      fn_name: "add_spam_reporter_for_coordination",
      payload: {
        base_coordination_hash: baseAddress,
        target_spam_reporter: targetAddress
      }
    });
    
    await pause(1200);
    
    // Bob gets the links again
    linksOutput = await bob.cells[0].callZome({
      zome_name: "coordinator",
      fn_name: "get_spam_reporters_for_coordination",
      payload: baseAddress
    });
    assert.equal(linksOutput.length, 1);


    // Bob gets the links in the inverse direction
    linksOutput = await bob.cells[0].callZome({
      zome_name: "coordinator",
      fn_name: "get_coordinations_for_spam_reporter",
      payload: targetAddress
    });
    assert.equal(linksOutput.length, 1);
    assert.deepEqual(baseRecord, linksOutput[0]);

    await alice.cells[0].callZome({
      zome_name: "coordinator",
      fn_name: "remove_spam_reporter_for_coordination",
      payload: {
        base_coordination_hash: baseAddress,
        target_spam_reporter: targetAddress
      }
    });
    
    await pause(1200);

    // Bob gets the links again
    linksOutput = await bob.cells[0].callZome({
      zome_name: "coordinator",
      fn_name: "get_spam_reporters_for_coordination",
      payload: baseAddress
    });
    assert.equal(linksOutput.length, 0);

    // Bob gets the links in the inverse direction
    linksOutput = await bob.cells[0].callZome({
      zome_name: "coordinator",
      fn_name: "get_coordinations_for_spam_reporter",
      payload: targetAddress
    });
    assert.equal(linksOutput.length, 0);

  });
});

