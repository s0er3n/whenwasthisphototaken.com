import { Component, createSignal } from "solid-js"

type Form = { url?: string, description?: string, year?: number, tags?: string, discord_name_tag: string }

let [form, setForm] = createSignal<Form>({})
let [consent, setConsent] = createSignal(false)

const AddImage: Component = () => {

  // async function submitForm() {
  //   if (form().url?.length && form().year && form().description && consent()) {
  //     let data = form()
  //     data.year = Number(data.year)
  //     try {
  //       let result = await fetch(`${import.meta.env.VITE_BACKEND_UPLOAD_URL}/image`, {
  //         method: "POST",
  //         body: JSON.stringify(form()),
  //         mode: 'no-cors',
  //         headers: { "Content-type": "application/json; charset=UTF-8" }
  //       },);
  //       if (result.ok) {
  //         alert("great success (thank you for adding an image <3)")
  //         setForm({ discord_name_tag: form().discord_name_tag })
  //         setConsent(false)
  //       }
  //     } catch (e) {
  //       alert(e)
  //     }
  //   } else {
  //     alert(
  //       "is everything filled out correctly?"
  //     )
  //   }
  // }

  // function setKeyInForm(key: string, value: any) {
  //   let form_cp = form()
  //   form_cp[key] = value.target.value
  //   setForm(form_cp)
  //   console.log(form())
  //
  // }
  return (<>
    <div class="flex flex-col justify-center items-center">
      <h1 class="text-4xl">Upload an Image</h1>
      <form
        // TODO: make this environment variable
        class="form-control "
        action="https://backend.whenwasthisphototaken.com/image" method="post" enctype="multipart/form-data">
        <label class="label" for="image"><span class="label-text">Image:</span></label>
        <input class="file-input" type="file" id="image" name="image" accept="image/*" required />

        <label class="label" for="year"><span class="label-text">Year:</span></label>
        <input placeholder="year the photo was taken" class="input input-bordered" type="number" min="1900" max="2022" required id="year" name="year" />

        <label class="label" for="description"><span class="label-text">Description:</span></label>
        <textarea placeholder="write a short description" class="textarea textarea-bordered" id="description" name="description" required></textarea>

        <label class="label input-bordered" for="tags"><span class="label-text">Tags:</span></label>
        <input placeholder="add some tags like: car portrait art" class="input input-bordered" type="text" id="tags" name="tags" />
        <label class="label" for="country"><span class="span">Country:</span></label>
        <select class="select select-bordered" id="country" name="country" required>
          <option value="">select country of photo</option>
          <option value="AFG">Afghanistan</option>
          <option value="ALA">Aland Islands</option>
          <option value="ALB">Albania</option>
          <option value="DZA">Algeria</option>
          <option value="ASM">American Samoa</option>
          <option value="AND">Andorra</option>
          <option value="AGO">Angola</option>
          <option value="AIA">Anguilla</option>
          <option value="ATA">Antarctica</option>
          <option value="ATG">Antigua and Barbuda</option>
          <option value="ARG">Argentina</option>
          <option value="ARM">Armenia</option>
          <option value="ABW">Aruba</option>
          <option value="AUS">Australia</option>
          <option value="AUT">Austria</option>
          <option value="AZE">Azerbaijan</option>
          <option value="BHS">Bahamas</option>
          <option value="BHR">Bahrain</option>
          <option value="BGD">Bangladesh</option>
          <option value="BRB">Barbados</option>
          <option value="BLR">Belarus</option>
          <option value="BEL">Belgium</option>
          <option value="BLZ">Belize</option>
          <option value="BEN">Benin</option>
          <option value="BMU">Bermuda</option>
          <option value="BTN">Bhutan</option>
          <option value="BOL">Bolivia</option>
          <option value="BES">Bonaire, Sint Eustatius and Saba</option>
          <option value="BIH">Bosnia and Herzegovina</option>
          <option value="BWA">Botswana</option>
          <option value="BVT">Bouvet Island</option>
          <option value="BRA">Brazil</option>
          <option value="IOT">British Indian Ocean Territory</option>
          <option value="BRN">Brunei Darussalam</option>
          <option value="BGR">Bulgaria</option>
          <option value="BFA">Burkina Faso</option>
          <option value="BDI">Burundi</option>
          <option value="KHM">Cambodia</option>
          <option value="CMR">Cameroon</option>
          <option value="CAN">Canada</option>
          <option value="CPV">Cape Verde</option>
          <option value="CYM">Cayman Islands</option>
          <option value="CAF">Central African Republic</option>
          <option value="TCD">Chad</option>
          <option value="CHL">Chile</option>
          <option value="CHN">China</option>
          <option value="CXR">Christmas Island</option>
          <option value="CCK">Cocos (Keeling) Islands</option>
          <option value="COL">Colombia</option>
          <option value="COM">Comoros</option>
          <option value="COG">Congo</option>
          <option value="COD">Congo, Democratic Republic of the Congo</option>
          <option value="COK">Cook Islands</option>
          <option value="CRI">Costa Rica</option>
          <option value="CIV">Cote D'Ivoire</option>
          <option value="HRV">Croatia</option>
          <option value="CUB">Cuba</option>
          <option value="CUW">Curacao</option>
          <option value="CYP">Cyprus</option>
          <option value="CZE">Czech Republic</option>
          <option value="DNK">Denmark</option>
          <option value="DJI">Djibouti</option>
          <option value="DMA">Dominica</option>
          <option value="DOM">Dominican Republic</option>
          <option value="ECU">Ecuador</option>
          <option value="EGY">Egypt</option>
          <option value="SLV">El Salvador</option>
          <option value="GNQ">Equatorial Guinea</option>
          <option value="ERI">Eritrea</option>
          <option value="EST">Estonia</option>
          <option value="ETH">Ethiopia</option>
          <option value="FLK">Falkland Islands (Malvinas)</option>
          <option value="FRO">Faroe Islands</option>
          <option value="FJI">Fiji</option>
          <option value="FIN">Finland</option>
          <option value="FRA">France</option>
          <option value="GUF">French Guiana</option>
          <option value="PYF">French Polynesia</option>
          <option value="ATF">French Southern Territories</option>
          <option value="GAB">Gabon</option>
          <option value="GMB">Gambia</option>
          <option value="GEO">Georgia</option>
          <option value="DEU">Germany</option>
          <option value="GHA">Ghana</option>
          <option value="GIB">Gibraltar</option>
          <option value="GRC">Greece</option>
          <option value="GRL">Greenland</option>
          <option value="GRD">Grenada</option>
          <option value="GLP">Guadeloupe</option>
          <option value="GUM">Guam</option>
          <option value="GTM">Guatemala</option>
          <option value="GGY">Guernsey</option>
          <option value="GIN">Guinea</option>
          <option value="GNB">Guinea-Bissau</option>
          <option value="GUY">Guyana</option>
          <option value="HTI">Haiti</option>
          <option value="HMD">Heard Island and Mcdonald Islands</option>
          <option value="VAT">Holy See (Vatican City State)</option>
          <option value="HND">Honduras</option>
          <option value="HKG">Hong Kong</option>
          <option value="HUN">Hungary</option>
          <option value="ISL">Iceland</option>
          <option value="IND">India</option>
          <option value="IDN">Indonesia</option>
          <option value="IRN">Iran, Islamic Republic of</option>
          <option value="IRQ">Iraq</option>
          <option value="IRL">Ireland</option>
          <option value="IMN">Isle of Man</option>
          <option value="ISR">Israel</option>
          <option value="ITA">Italy</option>
          <option value="JAM">Jamaica</option>
          <option value="JPN">Japan</option>
          <option value="JEY">Jersey</option>
          <option value="JOR">Jordan</option>
          <option value="KAZ">Kazakhstan</option>
          <option value="KEN">Kenya</option>
          <option value="KIR">Kiribati</option>
          <option value="PRK">Korea, Democratic People's Republic of</option>
          <option value="KOR">Korea, Republic of</option>
          <option value="XKX">Kosovo</option>
          <option value="KWT">Kuwait</option>
          <option value="KGZ">Kyrgyzstan</option>
          <option value="LAO">Lao People's Democratic Republic</option>
          <option value="LVA">Latvia</option>
          <option value="LBN">Lebanon</option>
          <option value="LSO">Lesotho</option>
          <option value="LBR">Liberia</option>
          <option value="LBY">Libyan Arab Jamahiriya</option>
          <option value="LIE">Liechtenstein</option>
          <option value="LTU">Lithuania</option>
          <option value="LUX">Luxembourg</option>
          <option value="MAC">Macao</option>
          <option value="MKD">Macedonia, the Former Yugoslav Republic of</option>
          <option value="MDG">Madagascar</option>
          <option value="MWI">Malawi</option>
          <option value="MYS">Malaysia</option>
          <option value="MDV">Maldives</option>
          <option value="MLI">Mali</option>
          <option value="MLT">Malta</option>
          <option value="MHL">Marshall Islands</option>
          <option value="MTQ">Martinique</option>
          <option value="MRT">Mauritania</option>
          <option value="MUS">Mauritius</option>
          <option value="MYT">Mayotte</option>
          <option value="MEX">Mexico</option>
          <option value="FSM">Micronesia, Federated States of</option>
          <option value="MDA">Moldova, Republic of</option>
          <option value="MCO">Monaco</option>
          <option value="MNG">Mongolia</option>
          <option value="MNE">Montenegro</option>
          <option value="MSR">Montserrat</option>
          <option value="MAR">Morocco</option>
          <option value="MOZ">Mozambique</option>
          <option value="MMR">Myanmar</option>
          <option value="NAM">Namibia</option>
          <option value="NRU">Nauru</option>
          <option value="NPL">Nepal</option>
          <option value="NLD">Netherlands</option>
          <option value="ANT">Netherlands Antilles</option>
          <option value="NCL">New Caledonia</option>
          <option value="NZL">New Zealand</option>
          <option value="NIC">Nicaragua</option>
          <option value="NER">Niger</option>
          <option value="NGA">Nigeria</option>
          <option value="NIU">Niue</option>
          <option value="NFK">Norfolk Island</option>
          <option value="MNP">Northern Mariana Islands</option>
          <option value="NOR">Norway</option>
          <option value="OMN">Oman</option>
          <option value="PAK">Pakistan</option>
          <option value="PLW">Palau</option>
          <option value="PSE">Palestinian Territory, Occupied</option>
          <option value="PAN">Panama</option>
          <option value="PNG">Papua New Guinea</option>
          <option value="PRY">Paraguay</option>
          <option value="PER">Peru</option>
          <option value="PHL">Philippines</option>
          <option value="PCN">Pitcairn</option>
          <option value="POL">Poland</option>
          <option value="PRT">Portugal</option>
          <option value="PRI">Puerto Rico</option>
          <option value="QAT">Qatar</option>
          <option value="REU">Reunion</option>
          <option value="ROM">Romania</option>
          <option value="RUS">Russian Federation</option>
          <option value="RWA">Rwanda</option>
          <option value="BLM">Saint Barthelemy</option>
          <option value="SHN">Saint Helena</option>
          <option value="KNA">Saint Kitts and Nevis</option>
          <option value="LCA">Saint Lucia</option>
          <option value="MAF">Saint Martin</option>
          <option value="SPM">Saint Pierre and Miquelon</option>
          <option value="VCT">Saint Vincent and the Grenadines</option>
          <option value="WSM">Samoa</option>
          <option value="SMR">San Marino</option>
          <option value="STP">Sao Tome and Principe</option>
          <option value="SAU">Saudi Arabia</option>
          <option value="SEN">Senegal</option>
          <option value="SRB">Serbia</option>
          <option value="SCG">Serbia and Montenegro</option>
          <option value="SYC">Seychelles</option>
          <option value="SLE">Sierra Leone</option>
          <option value="SGP">Singapore</option>
          <option value="SXM">Sint Maarten</option>
          <option value="SVK">Slovakia</option>
          <option value="SVN">Slovenia</option>
          <option value="SLB">Solomon Islands</option>
          <option value="SOM">Somalia</option>
          <option value="ZAF">South Africa</option>
          <option value="SGS">South Georgia and the South Sandwich Islands</option>
          <option value="SSD">South Sudan</option>
          <option value="ESP">Spain</option>
          <option value="LKA">Sri Lanka</option>
          <option value="SDN">Sudan</option>
          <option value="SUR">Suriname</option>
          <option value="SJM">Svalbard and Jan Mayen</option>
          <option value="SWZ">Swaziland</option>
          <option value="SWE">Sweden</option>
          <option value="CHE">Switzerland</option>
          <option value="SYR">Syrian Arab Republic</option>
          <option value="TWN">Taiwan, Province of China</option>
          <option value="TJK">Tajikistan</option>
          <option value="TZA">Tanzania, United Republic of</option>
          <option value="THA">Thailand</option>
          <option value="TLS">Timor-Leste</option>
          <option value="TGO">Togo</option>
          <option value="TKL">Tokelau</option>
          <option value="TON">Tonga</option>
          <option value="TTO">Trinidad and Tobago</option>
          <option value="TUN">Tunisia</option>
          <option value="TUR">Turkey</option>
          <option value="TKM">Turkmenistan</option>
          <option value="TCA">Turks and Caicos Islands</option>
          <option value="TUV">Tuvalu</option>
          <option value="UGA">Uganda</option>
          <option value="UKR">Ukraine</option>
          <option value="ARE">United Arab Emirates</option>
          <option value="GBR">United Kingdom</option>
          <option value="USA">United States</option>
          <option value="UMI">United States Minor Outlying Islands</option>
          <option value="URY">Uruguay</option>
          <option value="UZB">Uzbekistan</option>
          <option value="VUT">Vanuatu</option>
          <option value="VEN">Venezuela</option>
          <option value="VNM">Viet Nam</option>
          <option value="VGB">Virgin Islands, British</option>
          <option value="VIR">Virgin Islands, U.s.</option>
          <option value="WLF">Wallis and Futuna</option>
          <option value="ESH">Western Sahara</option>
          <option value="YEM">Yemen</option>
          <option value="ZMB">Zambia</option>
          <option value="ZWE">Zimbabwe</option>
        </select>
        <label class="flex items-center justify-center p-2">
          <input class="" type="radio" class="radio text-blue-600" name="license" value="free" required />
          <span class="ml-2 text-sm font-medium text-gray-700">This image is under a free license</span>
        </label>
        <div class="flex justify-center p-2">
          <input class="btn btn-primary btn-wide" type="submit" value="Upload" />
        </div>
      </form>
    </div>
  </>)

}












export default AddImage
