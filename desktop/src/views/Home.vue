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
  eventCb: Function | null= null;

  async created() {
    this.eventCb = (event: any) => {
      console.log(event);
      this.count = event.data.count;
    };
    native.on('event', this.eventCb);
    this.result = (await native.call()).count;
  }

  beforeDestroy() {
    if (this.eventCb) {
      native.off('event', this.eventCb);
    }
  }

  // get hello(): string {
  //   return native.hello();
  // }
}
</script>
