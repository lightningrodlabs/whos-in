import { assert, test } from "vitest";

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeDnaHash, fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createTwilioCredentials, sampleTwilioCredentials } from './common.js';

test('create TwilioCredentials', async () => {
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

    // Alice creates a TwilioCredentials
    const record: Record = await createTwilioCredentials(alice.cells[0]);
    assert.ok(record);
  });
});

test('create and read TwilioCredentials', async () => {
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

    const sample = await sampleTwilioCredentials(alice.cells[0]);

    // Alice creates a TwilioCredentials
    const record: Record = await createTwilioCredentials(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await pause(1200);

    // Bob gets the created TwilioCredentials
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "coordinator",
      fn_name: "get_twilio_credentials",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);
  });
});

test('create and update TwilioCredentials', async () => {
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

    // Alice creates a TwilioCredentials
    const record: Record = await createTwilioCredentials(alice.cells[0]);
    assert.ok(record);
        
    const originalActionHash = record.signed_action.hashed.hash;
 
    // Alice updates the TwilioCredentials
    let contentUpdate: any = await sampleTwilioCredentials(alice.cells[0]);
    let updateInput = {
      original_twilio_credentials_hash: originalActionHash,
      previous_twilio_credentials_hash: originalActionHash,
      updated_twilio_credentials: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "coordinator",
      fn_name: "update_twilio_credentials",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated TwilioCredentials
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "coordinator",
      fn_name: "get_twilio_credentials",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput0.entry as any).Present.entry) as any);

    // Alice updates the TwilioCredentials again
    contentUpdate = await sampleTwilioCredentials(alice.cells[0]);
    updateInput = { 
      original_twilio_credentials_hash: originalActionHash,
      previous_twilio_credentials_hash: updatedRecord.signed_action.hashed.hash,
      updated_twilio_credentials: contentUpdate,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "coordinator",
      fn_name: "update_twilio_credentials",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated TwilioCredentials
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "coordinator",
      fn_name: "get_twilio_credentials",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput1.entry as any).Present.entry) as any);
  });
});

test('create and delete TwilioCredentials', async () => {
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

    // Alice creates a TwilioCredentials
    const record: Record = await createTwilioCredentials(alice.cells[0]);
    assert.ok(record);
        
    // Alice deletes the TwilioCredentials
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "coordinator",
      fn_name: "delete_twilio_credentials",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await pause(1200);
        
    // Bob tries to get the deleted TwilioCredentials
    const readDeletedOutput = await bob.cells[0].callZome({
      zome_name: "coordinator",
      fn_name: "get_twilio_credentials",
      payload: record.signed_action.hashed.hash,
    });
    assert.notOk(readDeletedOutput);
  });
});
