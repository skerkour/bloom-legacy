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
import { Subscription } from 'rxjs';
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
  subscription: Subscription | null = null;

  async created() {
    this.subscription = native.notification.subscribe((event: any) => {
      console.log(event);
      this.count = event.data.count;
    });
    this.result = (await native.call()).count;
  }

  beforeDestroy() {
    if (this.subscription) {
      this.subscription.unsubscribe();
    }
  }
}
</script>
