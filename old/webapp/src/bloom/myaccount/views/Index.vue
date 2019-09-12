<template>
<div>
  <v-container>
  <v-layout row wrap justify-left>

    <v-flex xs12 v-if="initial_loading">
      <div class="elevation-0">
        <v-layout row wrap justify-center>
        <v-progress-circular
          :size="50"
          color="primary"
          indeterminate
        ></v-progress-circular>
      </v-layout>
      </div>
    </v-flex>

    <v-flex xs12 text-xs-center v-if="error">
      <v-alert icon="mdi-alert-circle" :value="error" type="error">
        {{ error }}
      </v-alert>
    </v-flex>



    <v-flex xs12 sm10 md8 xl7 v-if="me">
      <div class="elevation-0">
        <v-container grid-list-xl text-xs-left>
          <v-layout row wrap>

            <v-flex xs12>
              <div class="headline text-xs-left">
                Public Profile
              </div>
            </v-flex>

            <v-flex xs12 sm3>
              <v-subheader>Avatar</v-subheader>
            </v-flex>
            <v-flex xs12 sm9>
              <v-hover>
                <v-avatar class="pointer" slot-scope="{ hover }" @click="open_avatar_upload">
                  <v-expand-transition>
                    <div
                    v-if="hover"
              class="d-flex transition-fast-in-fast-out black darken-2 white--text v-avatar--reveal"
                    style="height: 100%;"
                    >
                    Update
                  </div>
                </v-expand-transition>
                <img v-if="me.avatar_url" :src="me.avatar_url">
                <img v-else src="/kernel/static/imgs/profile.jpg" alt="Avatar">
              </v-avatar>
            </v-hover>
            <v-progress-circular
            :size="50"
            color="primary"
            indeterminate
            v-if="is_loading"
            ></v-progress-circular>
          </v-flex>

          <v-flex hidden-xs-only sm3>
            <v-subheader>Username</v-subheader>
          </v-flex>
          <v-flex xs12 sm9>
            <v-text-field
              v-model="me.username"
              label="Username"
              prefix="@"
              disabled
            ></v-text-field>
          </v-flex>


          <v-flex xs12>
            <blm-myaccount-form-display-name
            :displayname="me.display_name"
            @update="update_display_name" />
          </v-flex>

          <v-flex xs12>
            <blm-myaccount-form-bio
            :bio="me.bio"
            @update="update_bio" />
          </v-flex>


          <v-flex xs12>
            <v-divider></v-divider>
          </v-flex>

          <v-flex xs12>
            <div class="headline text-xs-left">
              Personal Information
            </div>
          </v-flex>

          <v-flex xs12>
            <blm-myaccount-form-name
            :firstname="me.first_name"
            :lastname="me.last_name"
            @update="update_name" />
          </v-flex>


          <v-flex xs12>
            <blm-myaccount-form-email :email="me.email" @update="update_email" />
          </v-flex>


          <v-flex xs12><v-divider></v-divider></v-flex>

          <v-flex xs12>
            <div class="headline text-xs-left">
              Settings
            </div>
          </v-flex>


          <v-flex xs3>
            <v-subheader>Theme</v-subheader>
          </v-flex>
          <v-flex xs9>
            <v-switch
              :input-value="dark_mode"
              :label="`${dark_mode ? 'Dark' : 'Light'}`"
              v-on:change="on_dark_mode_changed"
            ></v-switch>
          </v-flex>


        </v-layout>
      </v-container>
    </div>
  </v-flex>

<input type="file" class="avatar-upload" ref="avatarupload" v-on:change="upload_avatar()"/>
</v-layout>
</v-container>
<blm-footer />
</div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import FormEmail from '../components/EmailForm.vue';
import FormName from '../components/NameForm.vue';
import DisplayNameForm from '../components/DisplayNameForm.vue';
import BioForm from '../components/BioForm.vue';
const { log } = require('@bloom42/astro');

interface Name {
  first_name: string;
  last_name: string;
}


@Component({
  components: {
    'blm-myaccount-form-bio': BioForm,
    'blm-myaccount-form-display-name': DisplayNameForm,
    'blm-myaccount-form-email': FormEmail,
    'blm-myaccount-form-name': FormName,
  },
})
export default class Index extends Vue {
  error = '';
  initial_loading = false;
  is_loading = false;
  me: any | null = null;
  vm = new Vue();

  created() {
    this.fetch_data();
  }

  get dark_mode() {
    return this.$store.state.dark_mode;
  }


  async fetch_data() {
    this.error = '';
    try {
      this.initial_loading = true;
      const res = await api.get(`${api.MYACCOUNT}/v1/me`);
      this.me = res;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.initial_loading = false;
    }
  }

  update_name(name: Name) {
    this.me.first_name = name.first_name;
    this.me.last_name = name.last_name;
  }

  update_display_name(me: any) {
    this.me.display_name = me.display_name;
    this.$store.commit('set_account', { display_name: me.display_name });
  }

  update_bio(me: any) {
    this.me.bio = me.bio;
  }

  update_email(email: string) {
    this.me.email = email;
  }

  open_avatar_upload() {
    const upload = this.$refs.avatarupload as HTMLElement;
    upload.click();
  }

  async upload_avatar() {
    const upload = this.$refs.avatarupload as any; // ugly

    if (upload.files.length < 1) {
      return;
    }
    const file = upload.files[0];

    this.error = '';
    const too_large = file.size > (3 * 1024 * 1024);
    if (too_large) {
      this.error = 'File size must be less or equal to 2MB';
      return;
    }
    const good_format = file.type === 'image/jpeg' || file.type === 'image/png';
    if (good_format !== true) {
      this.error = 'Image format must be png, jpg or jpeg';
      return;
    }
    try {
      this.is_loading = true;
      const formData = new FormData();
      formData.append('file', file);
      const res = await api.put(`${api.MYACCOUNT}/v1/me/avatar`, formData,
        {
          headers: { 'Content-Type': 'multipart/form-data' },
        });
      this.me.avatar_url = res.avatar_url;
      this.$store.commit('set_account', res);
      this.$toast.success('Success', { icon: 'mdi-check-circle' });
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  encodeFileBase64(file: any) {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.readAsDataURL(file);
      reader.onload = () => {
        if ((reader.result as string).indexOf('data:image/jpeg;base64,') !== -1) {
          resolve((reader.result as string).substring(23));
        } else {
          resolve((reader.result as string).substring(22));
        }
      };
      reader.onerror = (error) => reject(error);
    });
  }

  on_dark_mode_changed(value: boolean) {
    api.store_dark_mode(value);
  }
}
</script>

<style scoped lang="scss">
.v-avatar--reveal {
  align-items: center;
  bottom: 0;
  justify-content: center;
  opacity: .5;
  position: absolute;
  width: 100%;
  border-radius: 50%;
}

.v-avatar {
  border: 1px solid #444;
}

.avatar-upload{
  display: none;
}
</style>
