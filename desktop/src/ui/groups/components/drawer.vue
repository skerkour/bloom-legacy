<template>
  <v-list nav dense class="text-left">

    <v-list-item exact :to="`/groups/${this.$route.params.group_id}/members`">
      <v-list-item-icon>
        <v-icon>mdi-account-group</v-icon>
      </v-list-item-icon>
      <v-list-item-content>
        <v-list-item-title>Members</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

    <v-list-item exact :to="`/groups/${this.$route.params.group_id}/preferences`">
      <v-list-item-icon>
        <v-icon>mdi-settings</v-icon>
      </v-list-item-icon>
      <v-list-item-content>
        <v-list-item-title>Preferences</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

  </v-list>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class Drawer extends Vue {
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
