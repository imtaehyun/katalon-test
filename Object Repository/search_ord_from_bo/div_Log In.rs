<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Log In</name>
   <tag></tag>
   <elementGuidId>ca037604-00da-4b39-bb97-072eb216a8d7</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value></value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>ember680</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>ember-view</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

    
    Log In
  




  
  
      
    
    
    
    
  
      
    
      
    


  


  
              
                
                  
                

              
              
                Execute if/else when Element exists
              

          

              Katalon Studio
  Web Testing

    
      
    
    





  
  



    
      
        You have selected 0 posts.

  select all







cancel selecting

      


          May 20171 / 22May 2017Dec 2018


      
        

          
              


            

              Roxanne_FranklandMay '17I’m trying to test a page that randomly generates one of two objects each time the page is loaded. I’d like to use an if statement to execute a click action depending on which object is found. I tried this:
if (WebUI.verifyElementPresent(findTestObject(‘Objects/object1’)) == true) {
WebUI.click(findTestObject('Objects/object1_action))
} else {
WebUI.click(findTestObject(‘Objects/object2_action’))
}
where Objects/object1 is the id of one of the dynamically generated objects.
The problem with this is that if object1 is not found then the test fails. I need it to continue to the else block.
I’m probably doing something horribly wrong… hoping you can assist.
 createdMay '17last replyDec '1821replies1.5kviews15users14likes3links33Roxanne_FranklandMay '17Yay it works!
Thanks Roxanne_FranklandMay '17Thanks for your response. I amended the script as per your suggestion but am getting the following error when executing the test;
Test Cases/Test1 FAILED because (of) groovy.lang.MissingMethodException: No signature of method: static com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords.verifyElementPresent() is applicable for argument types: (com.kms.katalon.core.testobject.TestObject, com.kms.katalon.core.model.FailureHandling) values: [TestObject - Object Repository/Objects/object1, …]Prithviraj_SahooMay '17I am also facing the same problem. Same with try catch blocks. The actions which are scripted in try block are not executed again after exception is handled via the catch block and I had to write the scripts for the actions in catch block as well. Hope, we get some suggestions.5 months laterLiubov_ShalimovaOct '17The same issue. Can anyone tell what to do?duyluongKatalon DeveloperMay '17Sorry about my mistake. Follow this link 61, that keyword needs a timeout parameter.duyluongKatalon DeveloperMay '17Katalon provides FailureHandling 11 feature as an parameter of every keywords to deal with failure. In your case, the script should be:
if (WebUI.verifyElementPresent(findTestObject('Objects/object1'), FailureHandling.OPTIONAL)) {
WebUI.click(findTestObject('Objects/object1_action))
} else {
WebUI.click(findTestObject('Objects/object2_action'))
}38 months laterGowri_Kumar_AnnapantJan '18Thanks “duyluong 5” for the sample code.Tim_van_der_WeydenJan '18I have used the same structure as given here, but it doesn’t work for me, the execution is not going into the else part of the code. It is frustrating me a little because searching on the web keeps bringing me here. I even tried to add ‘== true’ in the condition, but no luck.Ajay_Kumar_KhareJan '18How we can add assert condition in Katalon?1 month laterMate_MrseFeb '18I’m having the same problem as @“Tim van der Weyden”. I tried all of the above solutions, but none worked.
Any progress?13 months laterChris_TrevarthenMay '18Expanding on the answer from duyluong 2 and putting it all together:
// Set how long to wait to interact with/verify an element on the screen.// Setting this to 0 seems to fall back to the project default, which could be 30 sec.// If you don't want to wait that long, set to an integer > 0.int timeout = 1// We can define the test objects first for re-use.// This does not actually try to find the object on the page, but only looks// for the definition of the object in the object repository.TestObject object1 = findTestObject('Objects/object1_action')TestObject object2 = findTestObject('Objects/object2_action')// FailureHandling.STOP_ON_FAILURE = Default behavior. Exits/fails the test when the element is not found// FailureHandling.CONTINUE_ON_FAILURE = reports that an error occurred during the run but doesn't exit; continues running the &quot;else&quot; clause// FailureHandling.OPTIONAL = no error reported; continues running the &quot;else&quot; clause
if (WebUI.verifyElementPresent(object1, timeout, FailureHandling.OPTIONAL)) {
    WebUI.click(object1)
} else {
    WebUI.click(object2)
}
92 months laterd11Jul '18thanks for the answers.
Note however that Katalon Studio freezes up if I use a high TIMEOUT value (300 secs).20 days laterUriel_VerstegenAug '18I’m following the same line of code, but it won’t let me use Else with my If statement. It gives me the error ‘expecting ‘}’, found ‘else’ @ line 144, column 15.’
But it wants me to put enough brackets there to end my entire block, and then I can’t have my else statement. Any thoughts? I can try to provide some codeChris_TrevarthenAug '18Hi Uriel,
A code sample would be helpful to figure out what’s going on.
At its simplest, an if/else block looks like:
if (logic) {
   do the true thing
} else {
    do the false thing
}

It’s possible that if you have complicated “logic” statements, that you forgot to close a “)” at the end of it. Also note the opening &quot;{&quot;at the end of the “if” line.
Some people find if/else statements easier to write out with more line breaks, just to be more clear:
if (logic) 
{
   do the true thing
} 
else 
{
    do the false thing
}
1Uriel_VerstegenAug '18It was as simple as missing a closing parenthesis, Chris. Thank you for pointing that out. I thought I had checked it.2 months laterVasudha_Vanol1Oct '18Hi,
I am facing same problem in this scenario. I used above code in my script but not succeed.
In my case; I have to search object 1 in the page and if object 1 is found than click on it and pop-up window will open to do next step. if object 1 is not found and object 2 is available in the page on that time have to click on object 2 and than click on object 3 and than after pop-up window will open to do next step.
Here, What happen in first case;
Object 1 found and click on it and pop-up window open and then script go to search object 2 and it is not available on pop-up window and throw error message.
Second Case:
When object 1 is not found in the page on that time script doesn’t switch to else statement but trying to search object 1 and script doesn’t stop at all.
FYI: Project Settings >> Execution >>Default
Default wait for element timeout (in seconds) = 3
Delay between actions (in secs) = 1
Default page load time out =  wait until the page is loaded
For second case, I try with 30 secs time limit for wait for element
Code:
int timeout = 1
TestObject object1 = findTestObject(‘Object Repository/Object 1’)
TestObject object2 = findTestObject(‘Object Repository/Object 2’)
if (WebUI.verifyElementPresent(object1, timeout, FailureHandling.OPTIONAL)){
WebUI.click(object1)
} else {
WebUI.click(object2)
WebUI.click(findTestObject('Object Repository/Object 3'))

}
‘Click on add new user’
WebUI.click(findTestObject(‘Object Repository/Object 4’))
Error Message:
10-10-2018 10:30:26 AM - [FAILED] - Unable to click on object ‘Object Repository/ClientPreManagement-Object 2)’ (Root cause: com.kms.katalon.core.webui.exception.WebElementNotFoundException: Web element with id: ‘Object Repository/ClientPreManagement- Object 2’ located by ‘By.xpath: //*[@id = ‘xpath’]’ not found)
10-10-2018 10:30:26 AM - [FAILED] - Test Cases/Client Pre-Management Module - 400/UAT-LL-470-Add or edit a client Finance Manager/Reject_Sub_ClientFinmgr FAILED because (of) (Stack trace: com.kms.katalon.core.exception.StepFailedException: Unable to click on object ‘Object Repository/object 2’ (Root cause: com.kms.katalon.core.webui.exception.WebElementNotFoundException: Web element with id: ‘Object Repository/object 2’ located by ‘By.xpath: //*[@id = xpath]’ not found)Chris_TrevarthenOct '18Hi Vasudha,
The error about not being able to find Object 2 with an xpath of “//*[@id = xpath]” tells me that the object might not be set up correctly in the Object Repository. Could you please share what Object 1 and Object 2 look like in the repository?
Thanks,
Chris2 months later

              

          
          

  

        
      

    

  share a link to this topic



  
  


  
    


  
    


  
    





  




  









  
    
      
          
            
          

        
          

        
      

      

      
  




    
  




   Invalid date




  Invalid date 





</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;ember680&quot;)</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <value>//div[@id='ember680']</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <value>//section[@id='main']/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Free Download'])[1]/following::div[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Services'])[1]/following::div[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <value>//div</value>
   </webElementXpaths>
</WebElementEntity>
