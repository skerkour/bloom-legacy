<template>
  <div class="about">
    <v-navigation-drawer
      v-model="drawer"
      mini-variant
      permanent
    >

    <v-list-item>
        <v-list-item-avatar>
          <v-img src="https://randomuser.me/api/portraits/men/85.jpg"></v-img>
        </v-list-item-avatar>

        <v-list-item-title>John Leider</v-list-item-title>
      </v-list-item>

      <v-divider></v-divider>

    </v-navigation-drawer>
    <div class="text-center">
      <v-btn>Button</v-btn>
    </div>
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';


@Component
export default class About extends Vue {
  // props
  // data
  drawer = true;
  items = [
    { title: 'Home', icon: 'mdi-home-city' },
    { title: 'My Account', icon: 'mdi-account' },
    { title: 'Users', icon: 'mdi-account-group-outline' },
  ];
  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style lang="scss" scoped>
</style>
