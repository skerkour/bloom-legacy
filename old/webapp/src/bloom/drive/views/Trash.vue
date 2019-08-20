<template>
  <blm-drive-trash />
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Trash from '../components/Trash.vue';


@Component({
  components: {
    'blm-drive-trash': Trash,
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
