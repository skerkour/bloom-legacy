<template>
  <blm-notes-archive />
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import Archive from '../components/Archive.vue';


@Component({
  components: {
    'blm-notes-archive': Archive,
  },
})
export default class Index extends Vue {
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
