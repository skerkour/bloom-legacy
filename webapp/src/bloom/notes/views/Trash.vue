<template>
  <blm-notes-trash />
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import Trash from '../components/Trash.vue';


@Component({
  components: {
    'blm-notes-trash': Trash,
  },
})
export default class TrashView extends Vue {
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
