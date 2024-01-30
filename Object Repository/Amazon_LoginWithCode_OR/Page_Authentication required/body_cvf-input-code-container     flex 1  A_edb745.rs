<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_cvf-input-code-container     flex 1  A_edb745</name>
   <tag></tag>
   <elementGuidId>62591c73-aba2-457e-9ef9-2de337ac0553</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>body.a-aui_72554-c.a-aui_accordion_a11y_role_354025-c.a-aui_killswitch_csa_logger_372963-c.a-aui_launch_2021_ally_fixes_392482-c.a-aui_pci_risk_banner_210084-c.a-aui_preload_261698-c.a-aui_rel_noreferrer_noopener_309527-c.a-aui_template_weblab_cache_333406-c.a-aui_tnr_v2_180836-c.a-meter-animate</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>50951418-9a0e-4010-808a-7f1a935e2df4</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>a-aui_72554-c a-aui_accordion_a11y_role_354025-c a-aui_killswitch_csa_logger_372963-c a-aui_launch_2021_ally_fixes_392482-c a-aui_pci_risk_banner_210084-c a-aui_preload_261698-c a-aui_rel_noreferrer_noopener_309527-c a-aui_template_weblab_cache_333406-c a-aui_tnr_v2_180836-c a-meter-animate</value>
      <webElementGuid>b2fb98c2-b328-4943-980e-8e906cf811a5</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>{}






  
    

    
      



  
  

  
  
    
    
      

    
      
    
  


    
  




  


  #cvf-input-code-container {
    flex: 1;
  }









Authentication required

{&quot;isClientDrivenOtpSendingEnabled&quot;:false,&quot;arbParam&quot;:&quot;be398eea-7478-43b3-a5ae-3ae99791c6b4&quot;}

           IN +918639326702 
Change            
            We’ve sent a One Time Password (OTP) to the mobile number above. Please enter it to complete verification



Enter OTP        
        





Please check your code and try again.
Please enter the verification code.

  
    






  P.when(&quot;A&quot;, &quot;codeResendTimer&quot;, &quot;ready&quot;).execute(function(A, codeResendTimer) {
    A.$(document).ready(function() {
      var timer = codeResendTimer.createTimer(0, 'A message with a One Time Password (OTP) has been sent to', &quot;You can now request a new code if needed.&quot;, &quot;timer&quot;, &quot;timer&quot;, false);
    });
  });


Continue


        Resend OTP


      Resend OTP










Verification required
It looks like you've requested a new OTP recently. Please wait one minute before getting a new OTP. OTPs you've already received won't work, so be sure to give the new OTP a moment to arrive.
You can now request a new code if needed.
Get new OTP



  P.when('A', 'ready').execute(function(A) {
    var $ = A.$;

    A.declarative('cvf-input-code-handler', 'keyup', function(event) {
      var VALID_DIGIT_REGEX = /^\d{6}$/;
      var $target = event.$target;

      if (VALID_DIGIT_REGEX.test($target.val())) {
        $(&quot;#verification-code-form&quot;).submit();
      }
    });

    $('.cvf-widget-link-resend').click(function() {
      $('.cvf-widget-form-resend').submit();
    });
  });



or

Sign in with your password  







  .auth-footer-separator {
    display: inline-block;
    width: 20px;
  }





  
    
      
    

    
      
        
          
        

        
      

      
        

        
          
            Conditions of Use
          
        
      

      
    
      
        
          
        

        
      

      
        

        
          
            Privacy Notice
          
        
      

      
    
      
        
          
        

        
      

      
        

        
          
            Help
          
        
      

      
    
  

  
    
      © 1996-2024, Amazon.com, Inc. or its affiliates
    
  



id(&quot;cvf-input-code&quot;)</value>
      <webElementGuid>6404b921-eaa5-4937-bd21-8b90ffd30df1</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;a-js a-audio a-video a-canvas a-svg a-drag-drop a-geolocation a-history a-webworker a-autofocus a-input-placeholder a-textarea-placeholder a-local-storage a-gradients a-transform3d a-touch-scrolling a-text-shadow a-text-stroke a-box-shadow a-border-radius a-border-image a-opacity a-transform a-transition a-ember a-audio a-video a-canvas a-svg a-drag-drop a-geolocation a-history a-webworker a-autofocus a-input-placeholder a-textarea-placeholder a-local-storage a-gradients a-transform3d a-touch-scrolling a-text-shadow a-text-stroke a-box-shadow a-border-radius a-border-image a-opacity a-transform a-transition&quot;]/body[@class=&quot;a-aui_72554-c a-aui_accordion_a11y_role_354025-c a-aui_killswitch_csa_logger_372963-c a-aui_launch_2021_ally_fixes_392482-c a-aui_pci_risk_banner_210084-c a-aui_preload_261698-c a-aui_rel_noreferrer_noopener_309527-c a-aui_template_weblab_cache_333406-c a-aui_tnr_v2_180836-c a-meter-animate&quot;]</value>
      <webElementGuid>ab653e85-72fa-4ebf-afb4-0bda36412a86</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>0e49bfb0-bc6e-47be-b1a6-53687924b140</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;{}






  
    

    
      



  
  

  
  
    
    
      

    
      
    
  


    
  




  


  #cvf-input-code-container {
    flex: 1;
  }









Authentication required

{&quot;isClientDrivenOtpSendingEnabled&quot;:false,&quot;arbParam&quot;:&quot;be398eea-7478-43b3-a5ae-3ae99791c6b4&quot;}

           IN +918639326702 
Change            
            We’ve sent a One Time Password (OTP) to the mobile number above. Please enter it to complete verification



Enter OTP        
        





Please check your code and try again.
Please enter the verification code.

  
    






  P.when(&quot;A&quot;, &quot;codeResendTimer&quot;, &quot;ready&quot;).execute(function(A, codeResendTimer) {
    A.$(document).ready(function() {
      var timer = codeResendTimer.createTimer(0, &quot; , &quot;'&quot; , &quot;A message with a One Time Password (OTP) has been sent to&quot; , &quot;'&quot; , &quot;, &quot;You can now request a new code if needed.&quot;, &quot;timer&quot;, &quot;timer&quot;, false);
    });
  });


Continue


        Resend OTP


      Resend OTP










Verification required
It looks like you&quot; , &quot;'&quot; , &quot;ve requested a new OTP recently. Please wait one minute before getting a new OTP. OTPs you&quot; , &quot;'&quot; , &quot;ve already received won&quot; , &quot;'&quot; , &quot;t work, so be sure to give the new OTP a moment to arrive.
You can now request a new code if needed.
Get new OTP



  P.when(&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ready&quot; , &quot;'&quot; , &quot;).execute(function(A) {
    var $ = A.$;

    A.declarative(&quot; , &quot;'&quot; , &quot;cvf-input-code-handler&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;keyup&quot; , &quot;'&quot; , &quot;, function(event) {
      var VALID_DIGIT_REGEX = /^\d{6}$/;
      var $target = event.$target;

      if (VALID_DIGIT_REGEX.test($target.val())) {
        $(&quot;#verification-code-form&quot;).submit();
      }
    });

    $(&quot; , &quot;'&quot; , &quot;.cvf-widget-link-resend&quot; , &quot;'&quot; , &quot;).click(function() {
      $(&quot; , &quot;'&quot; , &quot;.cvf-widget-form-resend&quot; , &quot;'&quot; , &quot;).submit();
    });
  });



or

Sign in with your password  







  .auth-footer-separator {
    display: inline-block;
    width: 20px;
  }





  
    
      
    

    
      
        
          
        

        
      

      
        

        
          
            Conditions of Use
          
        
      

      
    
      
        
          
        

        
      

      
        

        
          
            Privacy Notice
          
        
      

      
    
      
        
          
        

        
      

      
        

        
          
            Help
          
        
      

      
    
  

  
    
      © 1996-2024, Amazon.com, Inc. or its affiliates
    
  



id(&quot;cvf-input-code&quot;)&quot;) or . = concat(&quot;{}






  
    

    
      



  
  

  
  
    
    
      

    
      
    
  


    
  




  


  #cvf-input-code-container {
    flex: 1;
  }









Authentication required

{&quot;isClientDrivenOtpSendingEnabled&quot;:false,&quot;arbParam&quot;:&quot;be398eea-7478-43b3-a5ae-3ae99791c6b4&quot;}

           IN +918639326702 
Change            
            We’ve sent a One Time Password (OTP) to the mobile number above. Please enter it to complete verification



Enter OTP        
        





Please check your code and try again.
Please enter the verification code.

  
    






  P.when(&quot;A&quot;, &quot;codeResendTimer&quot;, &quot;ready&quot;).execute(function(A, codeResendTimer) {
    A.$(document).ready(function() {
      var timer = codeResendTimer.createTimer(0, &quot; , &quot;'&quot; , &quot;A message with a One Time Password (OTP) has been sent to&quot; , &quot;'&quot; , &quot;, &quot;You can now request a new code if needed.&quot;, &quot;timer&quot;, &quot;timer&quot;, false);
    });
  });


Continue


        Resend OTP


      Resend OTP










Verification required
It looks like you&quot; , &quot;'&quot; , &quot;ve requested a new OTP recently. Please wait one minute before getting a new OTP. OTPs you&quot; , &quot;'&quot; , &quot;ve already received won&quot; , &quot;'&quot; , &quot;t work, so be sure to give the new OTP a moment to arrive.
You can now request a new code if needed.
Get new OTP



  P.when(&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ready&quot; , &quot;'&quot; , &quot;).execute(function(A) {
    var $ = A.$;

    A.declarative(&quot; , &quot;'&quot; , &quot;cvf-input-code-handler&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;keyup&quot; , &quot;'&quot; , &quot;, function(event) {
      var VALID_DIGIT_REGEX = /^\d{6}$/;
      var $target = event.$target;

      if (VALID_DIGIT_REGEX.test($target.val())) {
        $(&quot;#verification-code-form&quot;).submit();
      }
    });

    $(&quot; , &quot;'&quot; , &quot;.cvf-widget-link-resend&quot; , &quot;'&quot; , &quot;).click(function() {
      $(&quot; , &quot;'&quot; , &quot;.cvf-widget-form-resend&quot; , &quot;'&quot; , &quot;).submit();
    });
  });



or

Sign in with your password  







  .auth-footer-separator {
    display: inline-block;
    width: 20px;
  }





  
    
      
    

    
      
        
          
        

        
      

      
        

        
          
            Conditions of Use
          
        
      

      
    
      
        
          
        

        
      

      
        

        
          
            Privacy Notice
          
        
      

      
    
      
        
          
        

        
      

      
        

        
          
            Help
          
        
      

      
    
  

  
    
      © 1996-2024, Amazon.com, Inc. or its affiliates
    
  



id(&quot;cvf-input-code&quot;)&quot;))]</value>
      <webElementGuid>63ab2021-0f6f-4219-b9a7-bf72b8611dbf</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
