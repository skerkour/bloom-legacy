<template>
  <blm-notes />
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import Notes from '../components/Notes.vue';

@Component({
  components: {
    'blm-notes': Notes,
  },
})
export default class Archive extends Vue {
  // props
  // data
  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style lang="scss" scoped>
</style>
