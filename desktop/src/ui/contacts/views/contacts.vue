<template>
  <v-layout fill-height>
    <v-col cols="4" class="pa-0">
      <v-toolbar elevation="0">
        <v-spacer />
        <v-tooltip bottom>
          <template v-slot:activator="{ on }">
            <v-btn icon v-on="on">
              <v-icon>mdi-plus</v-icon>
            </v-btn>
          </template>
          <span>New Contact</span>
        </v-tooltip>
      </v-toolbar>
      <div style="height: calc(100vh - 65px)" class="overflow-y-auto">
      </div>
    </v-col>

    <v-col cols="8" class="pa-0 blm-main-col">
      <blm-contact v-if="selectedContact" :contact="selectedContact" />
    </v-col>
  </v-layout>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import BlmContact from '../components/contact.vue';
import { Contact } from '../../../core/contacts';


@Component({
  components: {
    BlmContact,
  },
})
export default class BlmContacts extends Vue {
  // props
  // data
  selectedContact: Contact | null = null;
  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style lang="scss" scoped>
.v-toolbar {
  border-bottom: 1px solid rgba($color: #000000, $alpha: 0.1) !important;
  left: 0px !important;
}

.blm-main-col {
  border-left: 1px solid #dedede;
}
</style>
