<template>
  <blm-phaser-report />
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Report from '../components/Report.vue';


@Component({
  components: {
    'blm-phaser-report': Report,
  },
})
export default class ReportView extends Vue {
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
