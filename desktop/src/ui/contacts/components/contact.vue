<template>
  <div>
    <v-toolbar elevation="0" v-if="contact">
      <v-toolbar-title>{{ contact.firstName }} {{ contact.lastName }}</v-toolbar-title>

      <v-menu>
        <template v-slot:activator="{ on }">
          <v-btn icon v-on="on">
            <v-icon>mdi-dots-vertical</v-icon>
          </v-btn>
        </template>
        <v-list class="text-left">
          <v-list-item>
            <v-list-item-icon>
              <v-icon>mdi-delete</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Delete forever</v-list-item-title>
          </v-list-item>
        </v-list>
      </v-menu>
    </v-toolbar>
  </div>
</template>


<script lang="ts">
import { Component, Vue, Prop } from 'vue-property-decorator';
import { Contact } from '@/core/contacts';

@Component
export default class BlmContact extends Vue {
  // props
  @Prop({ type: Object, required: true }) contact!: Contact;

  // data
  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style lang="scss" scoped>
</style>
