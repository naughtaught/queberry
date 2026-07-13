!macro NSIS_HOOK_POSTINSTALL
  ; Create folders
  CreateDirectory "$INSTDIR\avatars"
  CreateDirectory "$INSTDIR\plugins"
  
  ; Grant permissions
  nsExec::ExecToLog 'icacls "$INSTDIR" /grant "Users:(OI)(CI)(F)" /T /Q'
  
  ; Tell the uninstaller about these folders
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\queberry" \
    "CustomFolders" "avatars,plugins"
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  ; Remove read-only attributes so deletion works
  nsExec::ExecToLog 'attrib -R "$INSTDIR\*.*" /S /D'
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  ; Clean up the entire directory
  RMDir /r "$INSTDIR"
  Delete "$INSTDIR\*.*"
  RMDir "$INSTDIR"
!macroend