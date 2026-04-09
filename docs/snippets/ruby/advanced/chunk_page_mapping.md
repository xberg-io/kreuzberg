Require 'Kreuzberg'

Config = Kreuzberg::ExtractionConfig.new(
chunking: Kreuzberg::ChunkingConfig.new(chunk_size: 500, overlap: 50),
pages: Kreuzberg::PageConfig.new(extract_pages: true)
)

Result = Kreuzberg.extract_file_sync("document.pdf", config: config)

Result.chunks&.each do |chunk|
if chunk.metadata.first_page
page_range = if chunk.metadata.first_page == chunk.metadata.last_page
"Page #{chunk.metadata.first_page}"
else
"Pages #{chunk.metadata.first_page}-#{chunk.metadata.last_page}"
end

    puts "Chunk: #{chunk.content[0..50]}... (#{page_range})"

End
end
