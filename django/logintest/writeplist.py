with open('test2.plist', 'w') as f:
    f.write('''
    <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <!-- array of downloads. -->
  <key>items</key>
  <array>
   <dict>
    <!-- an array of assets to download -->
     <key>assets</key>
      <array>
       <!-- software-package: the ipa to install. -->
        <dict>
         <!-- required. the asset kind. -->
          <key>kind</key>
          <string>software-package</string>
          <!-- optional. md5 every n bytes. will restart a chunk if md5 fails. -->
          <key>md5-size</key>
          <integer>10485760</integer>
          <!-- optional. array of md5 hashes for each "md5-size" sized chunk. -->
          <key>md5s</key>
          <array>
            <string>41fa64bb7a7cae5a46bfb45821ac8bba</string>
            <string>51fa64bb7a7cae5a46bfb45821ac8bba</string>
          </array>
          <!-- required. the URL of the file to download. -->
          <key>url</key>
          <string>https://www.example.com/apps/foo.ipa</string>
        </dict>
        <!-- display-image: the icon to display during download.-->
        <dict>
         <key>kind</key>
         <string>display-image</string>
         <!-- optional. indicates if icon needs shine effect applied. -->
         <key>needs-shine</key>
         <true/>
         <key>url</key>
         <string>https://www.example.com/image.57x57.png</string>
        </dict>
        <!-- full-size-image: the large 512x512 icon used by iTunes. -->
        <dict>
         <key>kind</key>
         <string>full-size-image</string>
         <!-- optional. one md5 hash for the entire file. -->
         <key>md5</key>
         <string>61fa64bb7a7cae5a46bfb45821ac8bba</string>
         <key>needs-shine</key>
         <true/>
         <key>url</key><string>https://www.example.com/image.512x512.jpg</string>
        </dict>
      </array>
<key>metadata</key>
      <dict>
       <!-- required -->
       <key>bundle-identifier</key>
       <string>com.example.fooapp</string>
       <!-- optional (software only) -->
       <key>bundle-version</key>
       <string>1.0</string>
       <!-- required. the download kind. -->
       <key>kind</key>
       <string>software</string>
       <!-- optional. displayed during download; typically company name -->
       <key>subtitle</key>
       <string>Apple</string>
       <!-- required. the title to display during the download. -->
       <key>title</key>
       <string>Example Corporate App</string>
      </dict>
    </dict>
  </array>
</dict>
</plist>
    ''')