# frozen_string_literal: true

RSpec.describe 'Embeddings Vector Generation' do
  describe 'embedding configuration validation' do
    it 'creates Embedding config with default values' do
      embedding = Kreuzberg::Config::Embedding.new

      expect(embedding.model).to be_a(Hash)
      expect(embedding.normalize).to be true
      expect(embedding.batch_size).to eq(32)
      expect(embedding.show_download_progress).to be false
      expect(embedding.cache_dir).to be_nil
    end

    it 'creates Embedding config with custom values' do
      embedding = Kreuzberg::Config::Embedding.new(
        model: { type: :preset, name: 'large' },
        normalize: false,
        batch_size: 64,
        show_download_progress: true,
        cache_dir: '/tmp/embeddings'
      )

      expect(embedding.model[:type]).to eq(:preset)
      expect(embedding.model[:name]).to eq('large')
      expect(embedding.normalize).to be false
      expect(embedding.batch_size).to eq(64)
      expect(embedding.show_download_progress).to be true
      expect(embedding.cache_dir).to eq('/tmp/embeddings')
    end

    it 'converts Embedding config to hash' do
      embedding = Kreuzberg::Config::Embedding.new(
        model: { type: :preset, name: 'balanced' },
        normalize: true,
        batch_size: 16
      )

      hash = embedding.to_h

      expect(hash).to be_a(Hash)
      expect(hash[:model]).to be_a(Hash)
      expect(hash[:normalize]).to be true
      expect(hash[:batch_size]).to eq(16)
    end

    it 'compacts nil values in embedding hash' do
      embedding = Kreuzberg::Config::Embedding.new(
        model: { type: :preset, name: 'balanced' },
        batch_size: 32
      )

      hash = embedding.to_h

      expect(hash).not_to have_key(:cache_dir)
      expect(hash).not_to have_key(:show_download_progress) unless hash[:show_download_progress] == false
    end

    it 'accepts hash model specification' do
      embedding = Kreuzberg::Config::Embedding.new(
        model: { type: :preset, name: 'balanced', dimension: 384 }
      )

      expect(embedding.model).to be_a(Hash)
      expect(embedding.model[:type]).to eq(:preset)
      expect(embedding.model[:name]).to eq('balanced')
    end

    it 'converts numeric batch_size to integer' do
      embedding = Kreuzberg::Config::Embedding.new(batch_size: '64')

      expect(embedding.batch_size).to eq(64)
      expect(embedding.batch_size).to be_a(Integer)
    end
  end

  describe 'Chunking with Embedding integration' do
    it 'integrates embedding with chunking configuration' do
      chunking = Kreuzberg::Config::Chunking.new(
        max_chars: 500,
        max_overlap: 100,
        embedding: Kreuzberg::Config::Embedding.new(
          model: { type: :preset, name: 'balanced' },
          normalize: true
        )
      )

      expect(chunking.embedding).to be_a(Kreuzberg::Config::Embedding)
      expect(chunking.embedding.normalize).to be true
    end

    it 'accepts embedding config in Chunking' do
      embedding = Kreuzberg::Config::Embedding.new(batch_size: 16)
      chunking = Kreuzberg::Config::Chunking.new(embedding: embedding)

      expect(chunking.embedding).to be_a(Kreuzberg::Config::Embedding)
      expect(chunking.embedding.batch_size).to eq(16)
    end

    it 'accepts embedding config as hash in Chunking' do
      chunking = Kreuzberg::Config::Chunking.new(
        embedding: { batch_size: 32, normalize: false }
      )

      expect(chunking.embedding).to be_a(Kreuzberg::Config::Embedding)
      expect(chunking.embedding.batch_size).to eq(32)
      expect(chunking.embedding.normalize).to be false
    end

    it 'converts chunking with embedding to hash' do
      chunking = Kreuzberg::Config::Chunking.new(
        max_chars: 600,
        embedding: Kreuzberg::Config::Embedding.new(batch_size: 24)
      )

      hash = chunking.to_h

      expect(hash).to be_a(Hash)
      expect(hash[:embedding]).to be_a(Hash)
      expect(hash[:embedding][:batch_size]).to eq(24)
    end

    it 'handles nil embedding in chunking' do
      chunking = Kreuzberg::Config::Chunking.new(
        max_chars: 500,
        embedding: nil
      )

      expect(chunking.embedding).to be_nil
      hash = chunking.to_h
      expect(hash[:embedding]).to be_nil
    end
  end
end
