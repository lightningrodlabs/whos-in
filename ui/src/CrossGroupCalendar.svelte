<script lang="ts">
    // Cross-group view: a merged, read-only calendar of YOUR coordinations across
    // every group that has whos-in installed.
    //
    // This is deliberately NOT the single-group app. Moss renders this view in
    // addition to the per-group applet-view, so anything with side effects here
    // runs a second time against a source chain that the applet-view is already
    // driving. That is what produced
    //
    //   Source chain error: Attempted to commit a bundle to the source chain, but
    //   the source chain head has moved since the bundle began
    //
    // when App.svelte rendered the whole UI in both views and both copies ran the
    // startup availability write. So the rule for this component is: reads only.
    // No create/update/delete zome calls, no init-type work, no shared stores that
    // the single-group app also writes.
    import { onMount } from 'svelte';
    import { decode } from "@msgpack/msgpack";
    import Calendar from '@event-calendar/core';
    import TimeGrid from '@event-calendar/time-grid';
    import DayGrid from '@event-calendar/day-grid';
    import List from '@event-calendar/list';
    import Interaction from '@event-calendar/interaction';
    import type { Coordination } from './whosin/coordinator/types';

    // ReadonlyMap<AppletHash, { appletClient, profilesClient }> from
    // weClient.renderInfo.applets, passed in rather than read from context so this
    // component never touches the single-group client.
    export let applets: Array<any> = [];
    export let weClient: any = undefined;

    let loading = true;
    let loadErrors: Array<string> = [];
    let events = [];
    let sourceCount = 0;

    const plugins = [TimeGrid, DayGrid, List, Interaction];

    // Distinct colour per source group, so a merged calendar stays legible.
    const palette = ['#5A7D9A', '#8A6552', '#6B7F5C', '#7A5C7F', '#9A7B4F', '#4F7A78'];

    function getLocalISOString(date: Date) {
        const offsetMs = date.getTimezoneOffset() * 60 * 1000;
        return new Date(date.getTime() - offsetMs).toISOString().slice(0, 19);
    }

    async function labelFor(appletHash, index: number): Promise<string> {
        // appletInfo gives us the applet name and the groups it belongs to. It is
        // best-effort: an older Moss host may not answer, and a federated applet
        // can belong to more than one group.
        try {
            const info = await weClient?.appletInfo(appletHash);
            if (!info) return `Group ${index + 1}`;
            const groups = (info.groupProfiles ?? []).map(g => g.name).filter(Boolean);
            return groups.length ? groups.join(' + ') : (info.appletName || `Group ${index + 1}`);
        } catch (e) {
            console.warn("cross-group: could not resolve applet info", e);
            return `Group ${index + 1}`;
        }
    }

    async function loadOneApplet(appletHash, clients, index: number) {
        const label = await labelFor(appletHash, index);
        // get_my_coordinations returns the coordination Records this agent
        // participates in, resolved through ParticipantToCoordroles ->
        // CoordroleToCoordinations. It is a read; it commits nothing.
        const records = await clients.appletClient.callZome({
            cap_secret: null,
            role_name: 'whosin',
            zome_name: 'coordinator',
            fn_name: 'get_my_coordinations',
            payload: null,
        });

        return (records ?? [])
            .map(record => {
                try {
                    return {
                        hash: record.signed_action.hashed.hash,
                        coordination: decode((record.entry as any).Present.entry) as Coordination,
                    };
                } catch (e) {
                    // A record we cannot decode should not lose us the whole group.
                    console.warn(`cross-group: undecodable coordination in ${label}`, e);
                    return undefined;
                }
            })
            .filter(x => x !== undefined)
            .filter(x => (x.coordination as any).starts_date && (x.coordination as any).title)
            .map(x => {
                const c = x.coordination as any;
                const start = new Date(c.starts_date / 1000);
                const end = c.ends_date ? new Date(c.ends_date / 1000) : start;
                return {
                    id: `${index}:${c.title}:${c.starts_date}`,
                    title: `${c.title}  ·  ${label}`,
                    start: getLocalISOString(start),
                    end: getLocalISOString(end),
                    editable: false,
                    allDay: false,
                    backgroundColor: palette[index % palette.length],
                    extendedProps: { group: label, description: c.description || '' },
                };
            });
    }

    onMount(async () => {
        const entries = applets ?? [];
        sourceCount = entries.length;

        // Loaded per group and settled independently: one unreachable or slow
        // group must not blank the whole merged calendar.
        const results = await Promise.allSettled(
            entries.map(([appletHash, clients], i) => loadOneApplet(appletHash, clients, i))
        );

        const merged = [];
        results.forEach((r, i) => {
            if (r.status === 'fulfilled') {
                merged.push(...r.value);
            } else {
                console.error(`cross-group: failed to load group ${i}`, r.reason);
                loadErrors = [...loadErrors, `Group ${i + 1} could not be loaded.`];
            }
        });

        events = merged.sort((a, b) => a.start.localeCompare(b.start));
        options = { ...options, events };
        loading = false;
    });

    let options = {
        view: 'listWeek',
        height: '100%',
        events: [],
        headerToolbar: {
            start: 'title',
            center: '',
            end: 'listWeek,dayGridMonth,timeGridWeek today prev,next',
        },
        // Read-only: no dateClick, no eventDrop, no select handlers.
        editable: false,
        selectable: false,
        nowIndicator: true,
        eventContent: (info) => ({ html: `<div class="cg-event">${info.event.title}</div>` }),
    };
</script>

<div class="cross-group">
    <header>
        <h2>My schedule across groups</h2>
        <p class="sub">
            {#if loading}
                Loading your coordinations from {sourceCount} group{sourceCount === 1 ? '' : 's'}…
            {:else}
                {events.length} coordination{events.length === 1 ? '' : 's'} across {sourceCount} group{sourceCount === 1 ? '' : 's'}
            {/if}
        </p>
    </header>

    {#each loadErrors as err}
        <p class="err">{err}</p>
    {/each}

    {#if !loading && events.length === 0 && loadErrors.length === 0}
        <p class="empty">
            You have not committed to any coordinations with a date yet. Open whos-in inside a
            group to join one.
        </p>
    {:else}
        <div class="cal">
            <Calendar {plugins} {options} />
        </div>
    {/if}
</div>

<style>
    .cross-group {
        display: flex;
        flex-direction: column;
        height: 100vh;
        padding: 16px;
        box-sizing: border-box;
    }
    header { margin-bottom: 12px; }
    h2 { margin: 0; font-size: 1.1rem; }
    .sub { margin: 4px 0 0; opacity: 0.7; font-size: 0.85rem; }
    .err {
        margin: 4px 0;
        padding: 6px 10px;
        border-radius: 4px;
        background: #fdeaea;
        color: #8a2020;
        font-size: 0.85rem;
    }
    .empty { opacity: 0.7; }
    .cal { flex: 1; min-height: 0; }
</style>
