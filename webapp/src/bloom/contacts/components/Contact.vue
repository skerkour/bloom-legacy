<template>
  <v-dialog
    v-model="show"
    @keydown.esc="close()"
    scrollable
    width="50%"
    :fullscreen="$vuetify.breakpoint.smAndDown"
  >
    <v-card>
      <v-card-title primary-title v-if="contact === null">
        <h3 class="headline mb-0">Create new contact</h3>
      </v-card-title>
      <v-card-title primary-title v-else>
        <h3 class="headline mb-0">
          {{ contact.first_name }} {{ contact.last_name }}
        </h3>
        <v-spacer />
        <v-tooltip bottom>
          <v-menu slot="activator">
            <v-btn slot="activator" flat color="blue-grey" icon>
              <v-icon>mdi-dots-vertical</v-icon>
            </v-btn>

            <v-list>
              <v-list-tile @click="delete_contact(contact)">Delete</v-list-tile>
            </v-list>
          </v-menu>
        <span>More actions</span>
        </v-tooltip>
      </v-card-title>
      <v-divider />
      <v-card-text>
        <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
          {{ error }}
        </v-alert>
        <v-container fluid grid-list-lg>
          <v-layout row wrap>

            <!-- name -->
            <v-flex xs6>
              <v-text-field
                label="First name"
                v-model="first_name"
                :readonly="viewing"
                prepend-icon="mdi-account"
              ></v-text-field>
            </v-flex>
            <v-flex xs6>
              <v-text-field
                label="Last name"
                v-model="last_name"
                :readonly="viewing"
              ></v-text-field>
            </v-flex>

            <!-- addresses: TODO -->

            <!-- organizations -->
            <v-flex xs6>
              <v-text-field
                label="Company"
                v-model="organizations[0].name"
                :readonly="viewing"
                prepend-icon="mdi-domain"
              ></v-text-field>
            </v-flex>
            <v-flex xs6>
              <v-text-field
                label="Job title"
                v-model="organizations[0].title"
                :readonly="viewing"
              ></v-text-field>
            </v-flex>


            <!-- phones -->
            <v-flex xs12>
              <v-list class="pt-0 pb-0">
                <v-list-tile v-for="(phone, index) in phones" :key="index" class="contacts-add-row">
                  <v-layout row wrap align-center>
                    <v-flex xs1 class="pl-0 pr-0">
                      <v-icon v-if="index === 0">mdi-phone</v-icon>
                    </v-flex>
                    <v-flex xs5>
                      <v-text-field
                        label="Phone"
                        v-model="phone.phone"
                        :readonly="viewing"
                      ></v-text-field>
                    </v-flex>
                    <v-flex xs4>
                      <v-autocomplete
                          :items="phone_labels"
                          v-model="phone.label"
                          label="Label"
                          single-line
                          :readonly="viewing"
                        >
                      </v-autocomplete>
                    </v-flex>
                    <v-flex xs1 class="pr-0 pl-0">
                      <v-tooltip bottom v-if="creating || updating">
                        <v-btn
                          flat
                          icon
                          small
                          slot="activator"
                          color="grey"
                          class="action-button"
                          @click="remove_phone(index)"
                        >
                          <v-icon>mdi-close</v-icon>
                        </v-btn>
                          <span>Remove</span>
                      </v-tooltip>
                    </v-flex>
                    <v-flex xs1 class="pl-0 pr-0">
                      <v-tooltip bottom
                        v-if="(creating || updating) && index === phones.length - 1"
                      >
                        <v-btn
                          flat
                          icon
                          small
                          slot="activator"
                          color="blue darken-2"
                          class="ml-0"
                          @click="add_phone"
                        >
                          <v-icon>mdi-plus-circle</v-icon>
                        </v-btn>
                        <span>Add</span>
                      </v-tooltip>
                    </v-flex>
                  </v-layout>
                </v-list-tile>
              </v-list>
            </v-flex>


            <!-- emails -->
            <v-flex xs12>
              <v-list class="pt-0 pb-0">
                <v-list-tile v-for="(email, index) in emails" :key="index" class="contacts-add-row">
                  <v-layout row wrap align-center>
                    <v-flex xs1 class="pl-0 pr-0">
                      <v-icon v-if="index === 0">mdi-email-outline</v-icon>
                    </v-flex>
                    <v-flex xs5>
                      <v-text-field
                        label="Email"
                        v-model="email.email"
                        :readonly="viewing"
                      ></v-text-field>
                    </v-flex>
                    <v-flex xs4>
                      <v-autocomplete
                          :items="email_labels"
                          v-model="email.label"
                          label="Label"
                          single-line
                          :readonly="viewing"
                        >
                      </v-autocomplete>
                    </v-flex>
                    <v-flex xs1 class="pr-0 pl-0">
                      <v-tooltip bottom v-if="creating || updating">
                        <v-btn
                          flat
                          icon
                          small
                          slot="activator"
                          color="grey"
                          class="action-button"
                          @click="remove_email(index)"
                        >
                          <v-icon>mdi-close</v-icon>
                        </v-btn>
                          <span>Remove</span>
                      </v-tooltip>
                    </v-flex>
                    <v-flex xs1 class="pl-0 pr-0">
                      <v-tooltip bottom
                        v-if="(creating || updating) && index === emails.length - 1"
                      >
                        <v-btn
                          flat
                          icon
                          small
                          slot="activator"
                          color="blue darken-2"
                          class="ml-0"
                          @click="add_email"
                        >
                          <v-icon>mdi-plus-circle</v-icon>
                        </v-btn>
                        <span>Add</span>
                      </v-tooltip>
                    </v-flex>
                  </v-layout>
                </v-list-tile>
              </v-list>
            </v-flex>


            <!-- websites -->
            <v-flex xs12>
              <v-list class="pt-0 pb-0">
                <v-list-tile v-for="(website, index) in websites" :key="index" class="contacts-add-row">
                  <v-layout row wrap align-center>
                    <v-flex xs1 class="pl-0 pr-0">
                      <v-icon v-if="index === 0">mdi-earth</v-icon>
                    </v-flex>
                    <v-flex xs5>
                      <v-text-field
                        label="Website"
                        v-model="website.website"
                        :readonly="viewing"
                      ></v-text-field>
                    </v-flex>
                    <v-flex xs4>
                      <v-autocomplete
                          :items="website_labels"
                          v-model="website.label"
                          label="Label"
                          single-line
                          :readonly="viewing"
                        >
                      </v-autocomplete>
                    </v-flex>
                    <v-flex xs1 class="pr-0 pl-0">
                      <v-tooltip bottom v-if="creating || updating">
                        <v-btn
                          flat
                          icon
                          small
                          slot="activator"
                          color="grey"
                          class="action-button"
                          @click="remove_website(index)"
                        >
                          <v-icon>mdi-close</v-icon>
                        </v-btn>
                          <span>Remove</span>
                      </v-tooltip>
                    </v-flex>
                    <v-flex xs1 class="pl-0 pr-0">
                      <v-tooltip bottom
                        v-if="(creating || updating) && index === websites.length - 1"
                      >
                        <v-btn
                          flat
                          icon
                          small
                          slot="activator"
                          color="blue darken-2"
                          class="ml-0"
                          @click="add_website"
                        >
                          <v-icon>mdi-plus-circle</v-icon>
                        </v-btn>
                        <span>Add</span>
                      </v-tooltip>
                    </v-flex>
                  </v-layout>
                </v-list-tile>
              </v-list>
            </v-flex>



            <!-- birthday -->
            <v-flex xs12>
              <v-menu
                ref="birthday_menu"
                :close-on-content-click="false"
                v-model="birthday_menu"
                :nudge-right="40"
                lazy
                transition="scale-transition"
                offset-y
                full-width
                min-width="290px"
              >
                <v-text-field
                  slot="activator"
                  v-model="birthday"
                  label="Birthday"
                  prepend-icon="mdi-calendar"
                  readonly
                ></v-text-field>
                <v-date-picker
                  ref="birthday_picker"
                  v-model="birthday"
                  :max="new Date().toISOString().substr(0, 10)"
                  min="1900-01-01"
                  @change="save_birthday"
                  :readonly="viewing"
                ></v-date-picker>
              </v-menu>
            </v-flex>


            <!-- notes -->
            <v-flex xs12>
              <v-textarea
                label="Notes"
                v-model="notes"
                :readonly="viewing"
                prepend-icon="mdi-calendar-text"
              ></v-textarea>
            </v-flex>

          </v-layout>
        </v-container>
      </v-card-text>
      <v-divider />
      <v-card-actions>
        <v-spacer></v-spacer>
        <div v-if="creating || updating">
          <v-btn flat @click="cancel" :disabled="is_loading">Cancel</v-btn>
          <v-btn color="primary" :loading="is_loading" @click="save_contact">
            Save
          </v-btn>
        </div>
        <div v-else-if="viewing">
          <v-btn flat @click="close" :disabled="is_loading">Close</v-btn>
          <v-btn color="primary" :loading="is_loading" @click="editing = true">
            Edit
          </v-btn>
        </div>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>


<script lang="ts">
import { Component, Prop, Vue, Watch } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';

const DEFAULT_EMAIL = { email: null, label: 'OTHER' };
const DEFAULT_WEBSITE = { website: null, label: 'OTHER' };
const DEFAULT_PHONE = { phone: null, label: 'OTHER' };
const DEFAULT_ORGANIZATION = { name: null, title: null };

@Component
export default class Contact extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) visible!: boolean;
  @Prop({ type: Object, default: null }) contact!: any | null;


  // data
  error = '';
  is_loading = false;
  editing = false;
  // contact data
  first_name: string | null = null;
  last_name: string | null = null;
  notes: string | null = null;
  birthday: any | null = null;
  birthday_menu = false;
  organizations: any[] = [Object.assign({}, DEFAULT_ORGANIZATION)];
  phones: any[] = [Object.assign({}, DEFAULT_PHONE)];
  emails: any[] = [Object.assign({}, DEFAULT_EMAIL)];
  websites: any[] = [Object.assign({}, DEFAULT_WEBSITE)];
  addresses: any[] = [];
  email_labels = [
    { text: 'Home', value: 'HOME'},
    { text: 'Work', value: 'WORK'},
    { text: 'Other', value: 'OTHER'},
  ];
  phone_labels = [
    { text: 'Home', value: 'HOME'},
    { text: 'Work', value: 'WORK'},
    { text: 'Other', value: 'OTHER'},
    { text: 'Mobile', value: 'MOBILE'},
    { text: 'Main', value: 'MAIN'},
    { text: 'Home fax', value: 'HOME_FAX'},
    { text: 'Work fax', value: 'WORK_FAX'},
  ];
  address_labels = [
    { text: 'Home', value: 'HOME'},
    { text: 'Work', value: 'WORK'},
    { text: 'Other', value: 'OTHER'},
  ];
  website_labels = [
    { text: 'Profile', value: 'PROFILE'},
    { text: 'Blog', value: 'BLOG'},
    { text: 'Home page', value: 'HOME_PAGE'},
    { text: 'Work', value: 'WORK'},
    { text: 'Other', value: 'OTHER'},
  ];

  // computed
  get show() {
    return this.visible;
  }

  set show(value) {
    if (!value) {
      this.$emit('close');
    }
  }

  get creating() {
    return this.visible && this.contact === null;
  }

  get updating() {
    return this.visible && this.contact !== null && this.editing;
  }

  get viewing() {
     return this.visible && this.contact && this.editing === false;
  }


  // lifecycle
  // watch
  @Watch('contact')
  on_contact_changed(contact: any) {
    if (contact !== null) {
      this.first_name = contact.first_name;
      this.last_name = contact.last_name;
      this.notes = contact.notes;
      this.birthday = contact.birthday;
      this.emails = contact.emails.length > 0
        ? contact.emails
        : [Object.assign({}, DEFAULT_EMAIL)];
      this.websites = contact.websites.length > 0
        ? contact.websites
        : [Object.assign({}, DEFAULT_WEBSITE)];
      this.phones = contact.phones.length > 0
        ? contact.phones
        : [Object.assign({}, DEFAULT_PHONE)];
      this.organizations = contact.organizations.length > 0
        ? contact.organizations
        : [Object.assign({}, DEFAULT_ORGANIZATION)];
    } else {
      this.clear_fields();
    }
  }

  @Watch('birthday_menu')
  on_birthday_menu_changed(birthday_menu: boolean) {
    if (birthday_menu) {
      this.$nextTick(() => ((this.$refs.birthday_picker) as any).activePicker = 'YEAR');
    }
  }


  // methods
  close() {
    this.show = false;
    this.birthday_menu = false;
    this.editing = false;
    this.error = '';
    this.is_loading = false;
    this.clear_fields();
  }

  cancel() {
    if (this.updating) {
      this.editing = false;
    } else {
      this.close();
    }
  }

  add_email() {
    this.emails.push(Object.assign({}, DEFAULT_EMAIL));
  }
  remove_email(index: number) {
    this.emails.splice(index, 1);
    if (this.emails.length === 0) {
      this.add_email();
    }
  }

  add_website() {
    this.websites.push(Object.assign({}, DEFAULT_WEBSITE));
  }
  remove_website(index: number) {
    this.websites.splice(index, 1);
    if (this.websites.length === 0) {
      this.add_website();
    }
  }

  add_phone() {
    this.phones.push(Object.assign({}, DEFAULT_PHONE));
  }
  remove_phone(index: number) {
    this.phones.splice(index, 1);
    if (this.phones.length === 0) {
      this.add_phone();
    }
  }

  clear_fields() {
      this.first_name = null;
      this.last_name = null;
      this.notes = null;
      this.birthday = null;
      this.emails = [Object.assign({}, DEFAULT_EMAIL)];
      this.websites = [Object.assign({}, DEFAULT_WEBSITE)];
      this.phones = [Object.assign({}, DEFAULT_PHONE)];
      this.organizations = [Object.assign({}, DEFAULT_ORGANIZATION)];
  }

  save_contact() {
    if (this.contact) {
      this.update_contact();
    } else {
      this.create_contact();
    }
  }

  save_birthday(date: any) {
    (this.$refs.birthday_menu as any).save(date);
  }

  async delete_contact(contact: any) {
    this.error = '';
    this.is_loading = true;
    try {
      await api.delete(`${api.CONTACTS}/v1/contacts/${contact.id}`);
      this.$emit('delete', contact);
      this.close();
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async create_contact() {
    this.error = '';
    this.is_loading = true;
    const emails = this.emails.filter((e) => e.email !== null);
    const websites = this.websites.filter((w) => w.website !== null);
    const phones = this.phones.filter((p) => p.phone !== null);
    const organizations = this.organizations.filter((o) => {
      return o.name !== null || o.title !== null;
    });

    const payload = {
      addresses: this.addresses,
      birthday: this.birthday,
      emails,
      first_name: this.first_name,
      last_name: this.last_name,
      notes: this.notes,
      organizations,
      phones,
      websites,
    };

    try {
      const res = await api.post(`${api.CONTACTS}/v1/contacts`, payload);
      this.$emit('create', res);
      this.close();
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async update_contact() {
    this.error = '';
    this.is_loading = true;
    const emails = this.emails.filter((e) => e.email !== null);
    const websites = this.websites.filter((w) => w.website !== null);
    const phones = this.phones.filter((p) => p.phone !== null);
    const organizations = this.organizations.filter((o) => {
      return o.name !== null || o.title !== null;
    });
    const contact_id = this.contact ? this.contact.id : undefined;

    const payload = {
      addresses: this.addresses,
      birthday: this.birthday,
      emails,
      first_name: this.first_name,
      last_name: this.last_name,
      notes: this.notes,
      organizations,
      phones,
      websites,
    };

    try {
      const res = await api.put(`${api.CONTACTS}/v1/contacts/${contact_id}`, payload);
      this.$emit('update', res);
      this.close();
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }
}
</script>


<style scoped lang="scss">
.v-card {
  border-radius: 8px;
}
</style>

<style lang="scss">
.contacts-add-row {
  .v-list__tile {
    padding-left: 8px;
  }
}
</style>
