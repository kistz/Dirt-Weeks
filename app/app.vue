<template>
  <div>
    <NuxtRouteAnnouncer />

    <p v-for = "event in events">
      {{ event }}
    </p>
  </div>
</template>



<script setup lang="ts">
import type { Identity } from 'spacetimedb';
import { DbConnection, Event } from './tourney-api-gen';

const events:Ref<Array<Event>>= ref([]);


onBeforeMount( () => {
  DbConnection.builder().onConnect((connection: DbConnection, identity: Identity, token: string) => { 

    connection.subscriptionBuilder()
      .onApplied((ctx) => {
        console.log("Spacetime client cache initialized.");
      })
      .onError(() => 
        console.log("that is yek"))
      .subscribe("SELECT * FROM server_events")

    connection.db.serverEvents.onInsert((ctx, row) => {
      events.value.push(row.typed)
    })

 }).onConnectError(() => { console.log("not good"); }).withUri("http://localhost:1234").withModuleName("stdbtest").build()
  


})


</script>