<template>
  <v-container fill-height fluid class="pa-0">
    <v-col cols="4" lg="3" class="pa-0 blm-left-col">

      <v-toolbar elevation="0">
        <v-spacer />
        <v-tooltip bottom>
          <template v-slot:activator="{ on }">
            <v-btn icon v-on="on" @click="newContact">
              <v-icon>mdi-account-plus</v-icon>
            </v-btn>
          </template>
          <span>New Contact</span>
        </v-tooltip>
      </v-toolbar>

      <div style="height: calc(100vh - 65px)" class="overflow-y-auto">
        <v-alert icon="mdi-alert-circle" type="error" dismissible :value="error !== ''">
          {{ error }}
        </v-alert>
        <v-list-item-group
          v-model="selectedContactIndex"
          @change="setSelectedContactIndex"
        >
          <v-list two-line class="pa-0">
            <template v-for="(contact, index) in contacts" class="blm-pointer">

              <v-list-item :key="`contact-${index}`">
                <v-list-item-content class="text-left">
                  <v-list-item-title>
                    {{ contact.data.firstName }} {{ contact.data.lastName }}
                  </v-list-item-title>
                  <!-- <v-list-item-subtitle>{{ note.body }}</v-list-item-subtitle> -->
                </v-list-item-content>
              </v-list-item>
              <v-divider v-if="index !== contacts.length - 1" :key="index"/>

            </template>
          </v-list>
        </v-list-item-group>
      </div>
    </v-col>

    <v-col cols="8" lg="9" class="pa-0">
      <blm-contact
        v-if="selectedContact"
        :contact="selectedContact"
        ref="contact"
        @deleted="contactDeleted"
      />
    </v-col>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import BlmContact from '../components/contact.vue';
import {
  CONTACT_TYPE,
  Contact,
  Method,
  CreateContactParams,
} from '@/core/contacts';
import core, { BlmObject, Contacts } from '@/core';


const DEFAULT_EMAIL = { value: '', label: 'Personal' };
const DEFAULT_WEBSITE = { value: '', label: 'Personal' };
const DEFAULT_PHONE = { value: '', label: 'Mobile' };
// const DEFAULT_ORGANIZATION = { name: '', title: '' };
const DEFAULT_ADDRESS = {
  city: '',
  country: '',
  label: 'Home',
  postalCode: '',
  street: '',
  state: '',
};


@Component({
  components: {
    BlmContact,
  },
})
export default class BlmContacts extends Vue {
  // props
  // data
  error = '';
  loading = false;
  contacts: BlmObject<Contact>[] = [];
  selectedContact: BlmObject<Contact> | null = null;
  selectedContactIndex: number | null = null;
  saveInterval: any | null = null;

  // computed
  // lifecycle
  async created() {
    await this.findContacts();
    this.saveInterval = setInterval(this.save, 2000);
  }

  async beforeDestroy() {
    await this.save();
  }

  destroyed() {
    if (this.saveInterval) {
      clearInterval(this.saveInterval);
    }
  }

  // watch
  // methods
  async save() {
    if (this.selectedContact) {
      if (this.selectedContact.id === '') {
        await this.createContact();
      } else {
        await this.updateContact();
      }
    }
  }

  async findContacts() {
    this.error = '';
    this.loading = true;

    try {
      const res = await core.call(Method.ListContacts, core.Empty);
      this.contacts = (res as Contacts).contacts;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }

  async setSelectedContactIndex(selected: number | undefined) {
    // save before changing / closing contact
    await this.save();

    if (selected === undefined || selected >= this.contacts.length) {
      this.selectedContactIndex = null;
      this.selectedContact = null;
    } else {
      this.selectedContact = this.contacts[selected];
      this.selectedContactIndex = selected;
    }
  }

  contactDeleted(deletedContact: BlmObject<Contact>) {
    const contacts = this.contacts
      .filter((cont: BlmObject<Contact>) => cont.id !== deletedContact.id);
    this.contacts = contacts;
    this.selectedContact = null;
    this.selectedContactIndex = null;
  }

  async newContact() {
    await this.setSelectedContactIndex(undefined);

    const newContact: BlmObject<Contact> = {
      id: '',
      createdAt: new Date(),
      updatedAt: new Date(),
      data: {
        nickname: '',
        prefix: '',
        suffix: '',
        birthday: null,
        firstName: '',
        lastName: '',
        notes: '',
        emails: [DEFAULT_EMAIL],
        phones: [DEFAULT_PHONE],
        online: [DEFAULT_WEBSITE],
        organizations: [],
        addresses: [DEFAULT_ADDRESS],
        deviceId: '',
        bloomUsername: '',
      },
      groupId: null,
      type: CONTACT_TYPE,
    };
    this.contacts = [newContact, ...this.contacts];
    await this.setSelectedContactIndex(0);
  }

  async createContact() {
    this.error = '';
    this.loading = true;
    const params: CreateContactParams = {
      birthday: core.toDateIsoString((this.$refs.contact as any).birthday),
      ...this.selectedContact!.data,
    };
    try {
      const res = await core.call(Method.CreateContact, params);
      this.selectedContactIndex = 0;
      this.selectedContact = res;
      this.contacts[this.selectedContactIndex!] = res;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }

  async updateContact() {
    this.error = '';
    this.loading = true;

    const contact = { ...this.selectedContact } as BlmObject<Contact>;
    contact.data.birthday = core.toDateIsoString((this.$refs.contact as any).birthday);

    try {
      await core.call(Method.UpdateContact, contact);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }
}
</script>


<style lang="scss" scoped>
.v-toolbar {
  border-bottom: 1px solid rgba($color: #000000, $alpha: 0.1) !important;
  left: 0px !important;
}

.blm-left-col {
  border-right: 1px solid #dedede;
}
</style>
