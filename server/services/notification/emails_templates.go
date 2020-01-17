package notification

const DefaultEmailTemplate = `
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Strict//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd">
<html data-editor-version="2" class="sg-campaigns" xmlns="http://www.w3.org/1999/xhtml">
  <head>
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1, minimum-scale=1, maximum-scale=1" /><!--[if !mso]><!-->
    <meta http-equiv="X-UA-Compatible" content="IE=Edge" /><!--<![endif]-->
    <!--[if (gte mso 9)|(IE)]>
    <xml>
    <o:OfficeDocumentSettings>
    <o:AllowPNG/>
    <o:PixelsPerInch>96</o:PixelsPerInch>
    </o:OfficeDocumentSettings>
    </xml>
    <![endif]-->
    <!--[if (gte mso 9)|(IE)]>
    <style type="text/css">
      body {width: 600px;margin: 0 auto;}
      table {border-collapse: collapse;}
      table, td {mso-table-lspace: 0pt;mso-table-rspace: 0pt;}
      img {-ms-interpolation-mode: bicubic;}
    </style>
    <![endif]-->

    <style type="text/css">
      body, p, div {
        font-family: arial;
        font-size: 14px;
      }
      body {
        color: #2d313c;
      }
      body a {
        color: #409EFF;
        text-decoration: none;
      }
      p { margin: 0; padding: 0; }
      table.wrapper {
        width:100% !important;
        table-layout: fixed;
        -webkit-font-smoothing: antialiased;
        -webkit-text-size-adjust: 100%;
        -moz-text-size-adjust: 100%;
        -ms-text-size-adjust: 100%;
      }
      img.max-width {
        max-width: 100% !important;
      }
      .column.of-2 {
        width: 50%;
      }
      .column.of-3 {
        width: 33.333%;
      }
      .column.of-4 {
        width: 25%;
      }
      @media screen and (max-width:480px) {
        .preheader .rightColumnContent,
        .footer .rightColumnContent {
            text-align: left !important;
        }
        .preheader .rightColumnContent div,
        .preheader .rightColumnContent span,
        .footer .rightColumnContent div,
        .footer .rightColumnContent span {
          text-align: left !important;
        }
        .preheader .rightColumnContent,
        .preheader .leftColumnContent {
          font-size: 80% !important;
          padding: 5px 0;
        }
        table.wrapper-mobile {
          width: 100% !important;
          table-layout: fixed;
        }
        img.max-width {
          height: auto !important;
          max-width: 480px !important;
        }
        a.bulletproof-button {
          display: block !important;
          width: auto !important;
          font-size: 80%;
          padding-left: 0 !important;
          padding-right: 0 !important;
        }
        .columns {
          width: 100% !important;
        }
        .column {
          display: block !important;
          width: 100% !important;
          padding-left: 0 !important;
          padding-right: 0 !important;
          margin-left: 0 !important;
          margin-right: 0 !important;
        }
      }
    </style>
    <!--user entered Head Start-->

     <!--End Head user entered-->
  </head>
  <body>
    <center class="wrapper" data-link-color="#409EFF" data-body-style="font-size: 14px; font-family: arial; color: #2d313c; background-color: #ffffff;">
      <div class="webkit">
        <table cellpadding="0" cellspacing="0" border="0" width="100%" class="wrapper" bgcolor="#ffffff">
          <tr>
            <td valign="top" bgcolor="#ffffff" width="100%">
              <table width="100%" role="content-container" class="outer" align="center" cellpadding="0" cellspacing="0" border="0">
                <tr>
                  <td width="100%">
                    <table width="100%" cellpadding="0" cellspacing="0" border="0">
                      <tr>
                        <td>
                          <!--[if mso]>
                          <center>
                          <table><tr><td width="600">
                          <![endif]-->
                          <table width="100%" cellpadding="0" cellspacing="0" border="0" style="width: 100%; max-width:600px;" align="center">
                            <tr>
                              <td role="modules-container" style="padding: 0px 0px 0px 0px; color: #2d313c; text-align: left;" bgcolor="#ffffff" width="100%" align="left">

    <table class="module preheader preheader-hide" role="module" data-type="preheader" border="0" cellpadding="0" cellspacing="0" width="100%"
           style="display: none !important; mso-hide: all; visibility: hidden; opacity: 0; color: transparent; height: 0; width: 0;">
      <tr>
        <td role="module-content">
          <p></p>
        </td>
      </tr>
    </table>

    <table class="wrapper" role="module" data-type="image" border="0" cellpadding="0" cellspacing="0" width="100%" style="table-layout: fixed;">
      <tr>
        <td style="font-size:6px;line-height:10px;padding:0px 0px 0px 0px;" valign="top" align="center">
          <a href="https://bloom.sh"><img class="max-width" style="display:block;color:#000000;text-decoration:none;font-family:Helvetica, arial, sans-serif;font-size:16px;max-width:20% !important;width:20%;height:auto !important;" src="https://marketing-image-production.s3.amazonaws.com/uploads/3ec7dab47d54770f15f78a4daa13a0360e006c1c529daa1ebdc72c6628b8dcf2579f977020f9dcb216e958ef41bcb04fbb9f1777a8e967ec8f3be7dafda34369.png" alt="Bloom logo" width="120" border="0"></a>
        </td>
      </tr>
    </table>

    <table class="module" role="module" data-type="text" border="0" cellpadding="0" cellspacing="0" width="100%" style="table-layout: fixed;">
      <tr>
        <td style="padding:0px 0px 10px 0px;text-align:inherit;"
            height="100%"
            valign="top"
            bgcolor="">
            <h3 style="text-align: center;"><span style="color:#999999;">{{subject}}</span></h3>
        </td>
      </tr>
    </table>

    <table class="module"
           role="module"
           data-type="divider"
           border="0"
           cellpadding="0"
           cellspacing="0"
           width="100%"
           style="table-layout: fixed;">
      <tr>
        <td style="padding:10px 0px 10px 0px;"
            role="module-content"
            height="100%"
            valign="top"
            bgcolor="">
          <table border="0"
                 cellpadding="0"
                 cellspacing="0"
                 align="center"
                 width="100%"
                 height="1px"
                 style="line-height:1px; font-size:1px;">
            <tr>
              <td
                style="padding: 0px 0px 1px 0px;"
                bgcolor="#cCc"></td>
            </tr>
          </table>
        </td>
      </tr>
    </table>

    <table class="module" role="module" data-type="text" border="0" cellpadding="0" cellspacing="0" width="100%" style="table-layout: fixed;">
      <tr>
        <td style="padding:10px 0px 10px 0px;text-align:inherit;"
            height="100%"
            valign="top"
            bgcolor="">
            <div style="text-align: left;"><%body%></div>
        </td>
      </tr>
    </table>

    <table class="module"
           role="module"
           data-type="divider"
           border="0"
           cellpadding="0"
           cellspacing="0"
           width="100%"
           style="table-layout: fixed;">
      <tr>
        <td style="padding:10px 0px 010px 0px;"
            role="module-content"
            height="100%"
            valign="top"
            bgcolor="">
          <table border="0"
                 cellpadding="0"
                 cellspacing="0"
                 align="center"
                 width="100%"
                 height="1px"
                 style="line-height:1px; font-size:1px;">
            <tr>
              <td
                style="padding: 0px 0px 1px 0px;"
                bgcolor="#CCC"></td>
            </tr>
          </table>
        </td>
      </tr>
    </table>

    <table class="module" role="module" data-type="text" border="0" cellpadding="0" cellspacing="0" width="100%" style="table-layout: fixed;">
      <tr>
        <td style="padding:18px 0px 18px 0px;line-height:22px;text-align:inherit;"
            height="100%"
            valign="top"
            bgcolor="">
            <div style="text-align: center;"><a href="https://bloom.sh">Bloom</a></div>
        </td>
      </tr>
    </table>

    <table class="module" role="module" data-type="social" align="center" border="0" cellpadding="0" cellspacing="0" width="100%" style="table-layout: fixed;">
      <tbody>
        <tr>
          <td valign="top" style="padding:0px 0px 0px 0px;font-size:6px;line-height:10px;">
            <table align="center">
              <tbody>
                <tr>

                  <td style="padding: 0px 5px;">
        <a role="social-icon-link"  href="https://twitter.com/42bloom" target="_blank" alt="Twitter"
          data-nolink="false"
          title="Twitter "
          style="-webkit-border-radius:2px;-moz-border-radius:2px;border-radius:2px;display:inline-block;background-color:#7AC4F7;">
          <img role="social-icon" alt="Twitter" title="Twitter "
            height="30"
            width="30"
            style="height: 30px, width: 30px"
            src="https://marketing-image-production.s3.amazonaws.com/social/white/twitter.png" />
        </a>
      </td>




                </tr>
              </tbody>
            </table>
          </td>
        </tr>
      </tbody>
    </table>

                              </td>
                            </tr>
                          </table>
                          <!--[if mso]>
                          </td></tr></table>
                          </center>
                          <![endif]-->
                        </td>
                      </tr>
                    </table>
                  </td>
                </tr>
              </table>
            </td>
          </tr>
        </table>
      </div>
    </center>
  </body>
</html>
`
