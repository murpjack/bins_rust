" Initialize the channel
if !exists('s:bindayJobId')
	let s:bindayJobId = 0
endif

" Constants for RPC messages.
let s:Show = 'show'

" The path to the binary that was created out of 'cargo build' or 'cargo build --release". This will generally be 'target/release/name'
let s:bin = '~/.config/nvim/vim-plug/bins_rust/target/debug/bins_rust'

" Entry point. Initialize RPC. If it succeeds, then attach commands to the `rpcnotify` invocations.
function! s:connect()
  let id = s:initRpc()
  
  if 0 == id
    echoerr "binday: cannot start rpc process"
  elseif -1 == id
    echoerr "binday: rpc process is not executable"
  else
    " Mutate our jobId variable to hold the channel ID
    let s:bindayJobId = id 
    
    call s:configureCommands()
  endif
endfunction

function! s:configureCommands()
  " command! -nargs=0 SpotifyCurrentSong :call s:rpc(s:CurrentSong)
  command! -nargs=+ BinDay :call s:show(<f-args>)
endfunction

function! s:show(...)
   call rpcnotify(s:bindayJobId, s:Show)
endfunction

" Initialize RPC
function! s:initRpc()
  if s:bindayJobId == 0
    let jobid = jobstart([s:bin], { 'rpc': v:true })
    return jobid
  else
    return s:bindayJobId
  endif
endfunction

call s:connect()
