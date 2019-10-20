<template>
  <v-container fluid>
    <h1>Chat</h1>
  </v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';


@Component
export default class Index extends Vue {
  // props
  // data
  // computed
  // lifecycle
  // watch
  // methods
}

// import { Component, Prop, Vue } from 'vue-property-decorator';
// import { Subscription } from 'rxjs';
// import Native from '@/native';

// @Component
// export default class Index extends Vue {
//   // props
//   // data
//   subscription: Subscription | null = null;
//   result = '';
//   count = 0;

//   // computed
//   // lifecycle
//   async created() {
//     this.subscription = Native.notification.subscribe((event: any) => {
//       console.log(event);
//       this.count = event.data.count;
//     });
//     this.result = (await Native.call()).count;
//   }

//   beforeDestroy() {
//     if (this.subscription) {
//       this.subscription.unsubscribe();
//     }
//   }

//   // watch
//   // methods
// }
</script>


<style lang="scss" scoped>
</style>
