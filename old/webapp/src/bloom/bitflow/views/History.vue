<template>
  <blm-bitflow-history />
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import History from '../components/History.vue';


@Component({
  components: {
    'blm-bitflow-history': History,
  },
})
export default class Devices extends Vue {
  // props
  // data
  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style scoped lang="scss">
</style>
