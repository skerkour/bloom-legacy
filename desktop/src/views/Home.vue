<template>
  <div class="home">
    <img alt="Vue logo" src="../assets/logo.png">
    <HelloWorld msg="Welcome to Your Vue.js + TypeScript App"/>
    <h1>{{ result }}</h1>
    <h1>{{ count }}</h1>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import HelloWorld from '@/components/HelloWorld.vue';
import native from '@/native';

// import MyEventEmitter from './event_emitter';

@Component({
  components: {
    HelloWorld,
  },
})
export default class Home extends Vue {
  // call: any = null;
  result = '';
  count = 0;

  async created() {
    native.on('event', (event: any) => { this.count = event.Tick.count; });
    this.result = (await native.call()).Tick.count;
  }

  // get hello(): string {
  //   return native.hello();
  // }
}
</script>
