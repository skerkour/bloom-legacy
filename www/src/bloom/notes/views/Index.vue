<template>
  <blm-notes-notes v-if="$store.state.session" />
  <blm-notes-landing v-else />
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Landing from '../components/Landing.vue';
import Notes from '../components/Notes.vue';


@Component({
  components: {
    'blm-notes-landing': Landing,
    'blm-notes-notes': Notes,
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
