<template>
  <div>
    <blm-hello-world />
    <blm-footer />
  </div>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import HelloWorld from '@/components/hello_world.vue';
import Footer from '@/components/footer.vue';


@Component({
  components: {
    'blm-hello-world': HelloWorld,
    'blm-footer': Footer,
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
