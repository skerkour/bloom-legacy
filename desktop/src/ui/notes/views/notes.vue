<template>
  <blm-notes />
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import BlmNotes from '../components/notes.vue';

@Component({
  components: {
    BlmNotes,
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
