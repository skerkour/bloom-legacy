package notification

const DEFAULT_EMAIL_TEMPLATE = `
<!DOCTYPE html>
<html>
  <head>
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1, minimum-scale=1, maximum-scale=1" />
  </head>
  <body>
  <table style="font-family: 'Asap', 'Helvetica Neue', Helvetica, Arial, 'Lucida Grande', sans-serif; box-sizing: border-box; border-collapse: separate !important; mso-table-lspace: 0pt; mso-table-rspace: 0pt; width: 100%;" width="100%">
  <tbody><tr>
    <td style="box-sizing: border-box; font-family: 'Asap', 'Helvetica Neue', Helvetica, Arial, 'Lucida Grande', sans-serif; font-size: 16px; vertical-align: top; display: block; Margin: 0 auto !important; max-width: 600px; padding: 10px; width: 600px;" width="600" valign="top">
      <div style="box-sizing: border-box; display: block; margin: 0 auto; max-width: 600px;">
        <table style="box-sizing: border-box; border-collapse: separate !important; mso-table-lspace: 0pt; mso-table-rspace: 0pt; width: 100%; background: #ffffff; border-radius: 3px;" width="100%">
          <tbody><tr style="border-collapse: collapse;">
            <td style="box-sizing: border-box; font-family: 'Asap', 'Helvetica Neue', Helvetica, Arial, 'Lucida Grande', sans-serif;  font-size: 16px; vertical-align: top; text-align: left; padding: 0;" valign="top" align="center">
              <!--
            <img style="-ms-interpolation-mode: bicubic; max-height: 100px;" alt="Bloom logo" src="https://kerkour.fr/bloom/imgs/logo_email.jpg">
            -->
            </td>
          </tr>
          <tr>
            <td style="box-sizing: border-box; font-family: 'Asap', 'Helvetica Neue', Helvetica, Arial, 'Lucida Grande', sans-serif;  font-size: 16px; vertical-align: top;" valign="top">
              <table style="box-sizing: border-box; border-collapse: separate !important; mso-table-lspace: 0pt; mso-table-rspace: 0pt; width: 100%;" width="100%">
                <tbody><tr>
                  <td style="box-sizing: border-box; font-family: 'Asap', 'Helvetica Neue', Helvetica, Arial, 'Lucida Grande', sans-serif;  font-size: 16px; vertical-align: top;" valign="top">
                    <h2 style="color: #151A2D !important; font-family: 'Asap', 'Helvetica Neue', Helvetica, Arial, 'Lucida Grande', sans-serif;  font-weight: bold; line-height: 1.4em; margin: 0; margin-bottom: 40px; margin-top: 10px; font-size: 24px;">{{ .Title }}</h2>
                    <p style="color: #333; font-family: 'Asap', 'Helvetica Neue', Helvetica, Arial, 'Lucida Grande', sans-serif; font-size: 16px; font-weight: normal; margin: 0; margin-bottom: 15px;">

{{ .Body }}

</p>

                  </td>
                </tr>
              </tbody></table>
            </td>
          </tr>
        </tbody></table>
      </div>
    </td>
  </tr>
  <tr>
    <td>
      <div style="box-sizing: border-box; display: block; margin: 0 auto; max-width: 600px; padding: 10px;">
        <table style="box-sizing: border-box; border-collapse: separate !important; mso-table-lspace: 0pt; mso-table-rspace: 0pt; width: 100%; color: #B3B3C1; font-size: 12px;" width="100%">
          <tbody><tr style="color: #B3B3C1; font-size: 12px;">
            <td style="box-sizing: border-box; font-family: 'Asap', 'Helvetica Neue', Helvetica, Arial, 'Lucida Grande', sans-serif;  vertical-align: top; font-size: 12px; color: #B3B3C1; text-align: left; padding: 20px 0; padding-top: 0px; border-top: 1px solid #B3B3C1;" valign="top" align="center">
              <p style="font-family: 'Asap', 'Helvetica Neue', Helvetica, Arial, 'Lucida Grande', sans-serif;  font-weight: normal; margin: 0; margin-bottom: 0px; color: #B3B3C1; font-size: 12px;">
                </p><div style="text-align: center;">
                <a href="https://bloom.sh">
                    <img style="-ms-interpolation-mode: bicubic; max-height: 100px;display: inline-block;" alt="Bloom logo" src="https://kerkour.fr/bloom/imgs/logo_email.jpg">
                  </a>
                    <br />
                  <a style="box-sizing: border-box; text-decoration: underline; color: #B3B3C1; font-size: 12px;" href="https://bloom.sh">bloom.sh</a> -
                  Follow us on Twitter: <a style="box-sizing: border-box; text-decoration: underline; color: #B3B3C1; font-size: 12px;" href="https://twitter.com/@42bloom">@42bloom</a>
                  and Mastodon: <a style="box-sizing: border-box; text-decoration: underline; color: #B3B3C1; font-size: 12px;" href="https://mastodon.social/@42bloom">@42bloom@mastodon.social</a>
                </div>
                <div>

                </div>
              <p></p>
            </td>
          </tr>
        </tbody></table>
      </div>
    </td>
  </tr>
</tbody></table>
  </body>
</html>
`
